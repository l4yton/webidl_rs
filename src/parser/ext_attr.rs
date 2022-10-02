use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{parser, Argument, ExtAttrValue, ExtendedAttribute, Parser};

impl Parser<Vec<ExtendedAttribute>> for ExtendedAttribute {
    fn parse(input: &str) -> IResult<&str, Vec<ExtendedAttribute>> {
        delimited(
            preceded(multispace0, tag("[")),
            separated_list0(
                delimited(multispace0, tag(","), multispace0),
                parse_single_ext_attr,
            ),
            preceded(multispace0, tag("]")),
        )(input)
    }
}

fn parse_single_ext_attr(input: &str) -> IResult<&str, ExtendedAttribute> {
    let (input, identifier) = parser::identifier(input)?;
    let (input, value) = opt(preceded(
        delimited(multispace0, tag("="), multispace0),
        ExtAttrValue::parse,
    ))(input)?;

    Ok((
        input,
        ExtendedAttribute {
            identifier: identifier.to_string(),
            value,
        },
    ))
}

impl Parser<ExtAttrValue> for ExtAttrValue {
    fn parse(input: &str) -> IResult<&str, ExtAttrValue> {
        let (input, value) = alt((
            parse_ext_attr_named_arg_list,
            parse_ext_attr_ident,
            parse_ext_attr_ident_list,
            parse_ext_attr_wildcard,
        ))(input)?;

        Ok((input, value))
    }
}

fn parse_ext_attr_named_arg_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = parser::identifier(input)?;
    let (input, arguments) = delimited(
        preceded(multispace0, tag("(")),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            Argument::parse,
        ),
        preceded(multispace0, tag(")")),
    )(input)?;

    Ok((
        input,
        ExtAttrValue::NamedArgumentList(crate::NamedArgumentList {
            identifier: identifier.to_string(),
            arguments,
        }),
    ))
}

fn parse_ext_attr_ident(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = parser::identifier(input)?;
    Ok((input, ExtAttrValue::Identifier(identifier.to_string())))
}

fn parse_ext_attr_ident_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifiers) = delimited(
        preceded(multispace0, tag("(")),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parser::identifier,
        ),
        preceded(multispace0, tag(")")),
    )(input)?;

    Ok((
        input,
        ExtAttrValue::IdentifierList(identifiers.iter().map(|s| s.to_string()).collect()),
    ))
}

fn parse_ext_attr_wildcard(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, _) = tag("*")(input)?;
    Ok((input, ExtAttrValue::Wildcard))
}
