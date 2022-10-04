use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{parser, Parser, RecordType, StandardType, Type, ExtendedAttribute, UnionType};

impl Parser<Type> for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        alt((parse_union, parse_wrapped_type, parse_standard_type))(input)
    }
}

fn parse_union(input: &str) -> IResult<&str, Type> {
    let (input, ext_attrs) =
        map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
    let (input, types) = delimited(
        terminated(tag("("), parser::multispace_or_comment0),
        separated_list1(
            delimited(
                parser::multispace_or_comment1,
                tag("or"),
                parser::multispace_or_comment1,
            ),
            Type::parse,
        ),
        preceded(parser::multispace_or_comment0, tag(")")),
    )(input)?;
    // Change this to simply return an error.
    assert!(types.len() > 1, "Found union with only a single type");

    Ok((input, Type::Union(UnionType { ext_attrs, types })))
}

fn parse_standard_type(input: &str) -> IResult<&str, Type> {
    let (input, ext_attrs) =
        map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
    let (input, primitive_type_with_space) = opt(alt((
        tag("unsigned short"),
        tag("unsigned long long"),
        tag("unsigned long"),
        tag("long long"),
        tag("unrestricted float"),
        tag("unrestricted double"),
    )))(input)?;

    if let Some(name) = primitive_type_with_space {
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;
        return Ok((
            input,
            Type::Standard(StandardType {
                ext_attrs,
                name: name.to_string(),
                nullable,
            }),
        ));
    }

    let (input, name) = parser::identifier(input)?;
    let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

    Ok((
        input,
        Type::Standard(StandardType {
            ext_attrs,
            name: name.to_string(),
            nullable,
        }),
    ))
}

fn parse_wrapped_type(input: &str) -> IResult<&str, Type> {
    alt((
        parse_sequence,
        parse_record,
        parse_promise,
        parse_frozen_array,
        parse_observable_array,
    ))(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag("sequence<"),
        delimited(
            parser::multispace_or_comment0,
            Type::parse,
            parser::multispace_or_comment0,
        ),
        tag(">"),
    )(input)?;

    Ok((input, Type::Sequence(Box::new(r#type))))
}

fn parse_record(input: &str) -> IResult<&str, Type> {
    let (input, (key, value)) = delimited(
        tag("record<"),
        separated_pair(
            delimited(
                parser::multispace_or_comment0,
                Type::parse,
                parser::multispace_or_comment0,
            ),
            tag(","),
            delimited(
                parser::multispace_or_comment0,
                Type::parse,
                parser::multispace_or_comment0,
            ),
        ),
        tag(">"),
    )(input)?;

    Ok((
        input,
        Type::Record(RecordType {
            key: Box::new(key),
            value: Box::new(value),
        }),
    ))
}

fn parse_promise(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag("Promise<"),
        delimited(
            parser::multispace_or_comment0,
            Type::parse,
            parser::multispace_or_comment0,
        ),
        tag(">"),
    )(input)?;

    Ok((input, Type::Promise(Box::new(r#type))))
}

fn parse_frozen_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag("FrozenArray<"),
        delimited(
            parser::multispace_or_comment0,
            Type::parse,
            parser::multispace_or_comment0,
        ),
        tag(">"),
    )(input)?;

    Ok((input, Type::FrozenArray(Box::new(r#type))))
}

fn parse_observable_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag("ObservableArray<"),
        delimited(
            parser::multispace_or_comment0,
            Type::parse,
            parser::multispace_or_comment0,
        ),
        tag(">"),
    )(input)?;

    Ok((input, Type::ObservableArray(Box::new(r#type))))
}
