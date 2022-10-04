use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{parser, Argument, ExtAttrValue, ExtendedAttribute, Parser};

impl Parser<Vec<ExtendedAttribute>> for ExtendedAttribute {
    fn parse(input: &str) -> IResult<&str, Vec<ExtendedAttribute>> {
        delimited(
            terminated(tag("["), parser::multispace_or_comment0),
            separated_list0(
                delimited(
                    parser::multispace_or_comment0,
                    tag(","),
                    parser::multispace_or_comment0,
                ),
                parse_single_ext_attr,
            ),
            preceded(parser::multispace_or_comment0, tag("]")),
        )(input)
    }
}

fn parse_single_ext_attr(input: &str) -> IResult<&str, ExtendedAttribute> {
    let (input, identifier) = parser::identifier(input)?;
    let (input, value) = opt(alt((
        preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("="),
                parser::multispace_or_comment0,
            ),
            ExtAttrValue::parse,
        ),
        // This is deprecated, but was used by: `Constructor(double x, double y)`.
        // Although this isn't technically a value, we parse the arguments as such.
        parse_ext_attr_arg_list,
    )))(input)?;

    Ok((
        input,
        ExtendedAttribute {
            identifier: identifier.to_string(),
            value,
        },
    ))
}

fn parse_ext_attr_arg_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, arguments) = preceded(parser::multispace_or_comment0, Argument::parse)(input)?;
    Ok((input, ExtAttrValue::ArgumentList(arguments)))
}

impl Parser<ExtAttrValue> for ExtAttrValue {
    fn parse(input: &str) -> IResult<&str, ExtAttrValue> {
        let (input, value) = alt((
            parse_ext_attr_named_arg_list,
            parse_ext_attr_ident,
            parse_ext_attr_ident_list,
            parse_ext_attr_wildcard,
            // Not in spec, but used: string in quotes.
            parse_ext_attr_in_quotes,
        ))(input)?;

        Ok((input, value))
    }
}

fn parse_ext_attr_named_arg_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = parser::identifier(input)?;
    let (input, arguments) = preceded(parser::multispace_or_comment0, Argument::parse)(input)?;

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
        terminated(tag("("), parser::multispace_or_comment0),
        separated_list0(
            delimited(
                parser::multispace_or_comment0,
                tag(","),
                parser::multispace_or_comment0,
            ),
            parser::identifier,
        ),
        preceded(parser::multispace_or_comment0, tag(")")),
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

fn parse_ext_attr_in_quotes(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, ExtAttrValue::Identifier(identifier.to_string())))
}
