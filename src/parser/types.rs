use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::{parser, Parser, Record, StandardType, Type};

impl Parser<Type> for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        alt((parse_union, parse_wrapped_type, parse_standard_type))(input)
    }
}

fn parse_union(input: &str) -> IResult<&str, Type> {
    let (input, types) = delimited(
        tag("("),
        separated_list1(delimited(multispace1, tag("or"), multispace1), Type::parse),
        tag(")"),
    )(input)?;
    assert!(types.len() > 1, "Found union with only a single type");

    Ok((input, Type::Union(types)))
}

fn parse_standard_type(input: &str) -> IResult<&str, Type> {
    let (input, primitive_type_with_space) = opt(alt((
        tag("unsigned short"),
        tag("unsigned long"),
        tag("long long"),
        tag("unsigned long long"),
        tag("unrestricted float"),
        tag("unrestricted double"),
    )))(input)?;

    if let Some(name) = primitive_type_with_space {
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;
        return Ok((
            input,
            Type::Standard(StandardType {
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
        tag_no_case("sequence<"),
        delimited(multispace0, Type::parse, multispace0),
        tag(">"),
    )(input)?;

    Ok((input, Type::Sequence(Box::new(r#type))))
}

fn parse_record(input: &str) -> IResult<&str, Type> {
    let (input, (key, value)) = delimited(
        tag_no_case("record<"),
        separated_pair(
            delimited(multispace0, Type::parse, multispace0),
            tag(","),
            delimited(multispace0, Type::parse, multispace0),
        ),
        tag(">"),
    )(input)?;

    Ok((
        input,
        Type::Record(Record {
            key: Box::new(key),
            value: Box::new(value),
        }),
    ))
}

fn parse_promise(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag_no_case("promise<"),
        delimited(multispace0, Type::parse, multispace0),
        tag(">"),
    )(input)?;

    Ok((input, Type::Promise(Box::new(r#type))))
}

fn parse_frozen_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag_no_case("frozenarray<"),
        delimited(multispace0, Type::parse, multispace0),
        tag(">"),
    )(input)?;

    Ok((input, Type::FrozenArray(Box::new(r#type))))
}

fn parse_observable_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = delimited(
        tag_no_case("observablearray<"),
        delimited(multispace0, Type::parse, multispace0),
        tag(">"),
    )(input)?;

    Ok((input, Type::ObservableArray(Box::new(r#type))))
}
