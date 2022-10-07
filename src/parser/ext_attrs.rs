use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{parser, ExtAttrValue, ExtendedAttribute, NamedArgumentList, Parser};

fn parse_ext_attr_arg_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, arguments) =
        preceded(parser::multispace_or_comment0, parser::parse_arguments)(input)?;
    Ok((input, ExtAttrValue::ArgumentList(arguments)))
}

fn parse_ext_attr_ident(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = parser::parse_identifier(input)?;
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
            alt((
                map(parser::parse_identifier, |s| s.to_string()),
                // Identifiers in a list may also be in quotes.
                parser::parse_quoted_string,
            )),
        ),
        preceded(parser::multispace_or_comment0, tag(")")),
    )(input)?;

    Ok((input, ExtAttrValue::IdentifierList(identifiers)))
}

fn parse_ext_attr_wildcard(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, _) = tag("*")(input)?;
    Ok((input, ExtAttrValue::Wildcard))
}

fn parse_ext_attr_in_quotes(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, identifier) = parser::parse_quoted_string(input)?;
    Ok((input, ExtAttrValue::Identifier(identifier)))
}

impl Parser<ExtendedAttribute> for ExtendedAttribute {
    fn parse(input: &str) -> IResult<&str, ExtendedAttribute> {
        let (input, identifier) = map(parser::parse_identifier, |s| s.to_string())(input)?;
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

        Ok((input, ExtendedAttribute { identifier, value }))
    }
}

impl Parser<ExtAttrValue> for ExtAttrValue {
    fn parse(input: &str) -> IResult<&str, ExtAttrValue> {
        alt((
            map(NamedArgumentList::parse, ExtAttrValue::NamedArgumentList),
            parse_ext_attr_ident,
            parse_ext_attr_ident_list,
            parse_ext_attr_wildcard,
            // Not in spec, but used: string in quotes.
            parse_ext_attr_in_quotes,
        ))(input)
    }
}

impl Parser<NamedArgumentList> for NamedArgumentList {
    fn parse(input: &str) -> IResult<&str, NamedArgumentList> {
        let (input, identifier) = map(parser::parse_identifier, |s| s.to_string())(input)?;
        let (input, arguments) =
            preceded(parser::multispace_or_comment0, parser::parse_arguments)(input)?;

        Ok((
            input,
            NamedArgumentList {
                identifier,
                arguments,
            },
        ))
    }
}
