use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{parser, ExtAttrValue, ExtendedAttribute, NamedArgumentList};

fn parse_ext_attr_ident_list(input: &str) -> IResult<&str, ExtAttrValue> {
    let (input, _) = terminated(tag("("), parser::multispace_or_comment0)(input)?;
    let (input, identifiers) = separated_list0(
        delimited(
            parser::multispace_or_comment0,
            tag(","),
            parser::multispace_or_comment0,
        ),
        alt((parser::parse_identifier, parser::parse_quoted_string)),
    )(input)?;
    let (input, _) = preceded(parser::multispace_or_comment0, tag(")"))(input)?;

    Ok((input, ExtAttrValue::IdentifierList(identifiers)))
}

impl ExtendedAttribute {
    pub(crate) fn parse(input: &str) -> IResult<&str, ExtendedAttribute> {
        let (input, identifier) = parser::parse_identifier(input)?;
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
            map(
                preceded(parser::multispace_or_comment0, parser::parse_arguments),
                ExtAttrValue::ArgumentList,
            ),
        )))(input)?;

        Ok((input, ExtendedAttribute { identifier, value }))
    }
}

impl ExtAttrValue {
    pub(crate) fn parse(input: &str) -> IResult<&str, ExtAttrValue> {
        alt((
            map(NamedArgumentList::parse, ExtAttrValue::NamedArgumentList),
            map(parser::parse_identifier, ExtAttrValue::Identifier),
            parse_ext_attr_ident_list,
            map(tag("*"), |_| ExtAttrValue::Wildcard),
            map(parser::parse_quoted_string, ExtAttrValue::Identifier),
        ))(input)
    }
}

impl NamedArgumentList {
    pub(crate) fn parse(input: &str) -> IResult<&str, NamedArgumentList> {
        let (input, identifier) = parser::parse_identifier(input)?;
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
