use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{
    parser, CallbackFunction, CallbackInterface, Definition, Dictionary, Enumeration, Includes,
    Interface, InterfaceMixin, Namespace, Parser, Type, Typedef,
};

fn parse_check_is_partial(input: &str) -> IResult<&str, bool> {
    map(
        opt(delimited(
            parser::multispace_or_comment0,
            tag("partial"),
            parser::multispace_or_comment1,
        )),
        |o| o.is_some(),
    )(input)
}

fn parse_optional_inheritance(input: &str) -> IResult<&str, Option<String>> {
    map(
        opt(preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(":"),
                parser::multispace_or_comment0,
            ),
            parser::parse_identifier,
        )),
        |o| o.map(|s| s.to_string()),
    )(input)
}

fn parse_definition_identifier<'a>(
    input: &'a str,
    definition_tag: &str,
) -> IResult<&'a str, String> {
    map(
        preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(definition_tag),
                parser::multispace_or_comment1,
            ),
            parser::parse_identifier,
        ),
        |s| s.to_string(),
    )(input)
}

impl Parser<Definition> for Definition {
    fn parse(input: &str) -> IResult<&str, Definition> {
        alt((
            Interface::parse,
            InterfaceMixin::parse,
            Includes::parse,
            CallbackInterface::parse,
            Namespace::parse,
            Dictionary::parse,
            Enumeration::parse,
            CallbackFunction::parse,
            Typedef::parse,
        ))(input)
    }
}

impl Parser<Definition> for Interface {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "interface")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        Ok((
            input,
            Definition::Interface(Interface {
                ext_attrs,
                partial,
                identifier,
                inheritance,
                members,
            }),
        ))
    }
}

impl Parser<Definition> for InterfaceMixin {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "interface mixin")?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        Ok((
            input,
            Definition::InterfaceMixin(InterfaceMixin {
                ext_attrs,
                partial,
                identifier,
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Includes {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, (interface, mixin)) = map(
            separated_pair(
                preceded(parser::multispace_or_comment0, parser::parse_identifier),
                delimited(
                    parser::multispace_or_comment1,
                    tag("includes"),
                    parser::multispace_or_comment1,
                ),
                parser::parse_identifier,
            ),
            |(s0, s1)| (s0.to_string(), s1.to_string()),
        )(input)?;

        Ok((
            input,
            Definition::Includes(Includes {
                ext_attrs,
                interface,
                mixin,
            }),
        ))
    }
}

impl Parser<Definition> for CallbackInterface {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, identifier) = parse_definition_identifier(input, "callback interface")?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        Ok((
            input,
            Definition::CallbackInterface(CallbackInterface {
                ext_attrs,
                identifier,
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Namespace {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "namespace")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        Ok((
            input,
            Definition::Namespace(Namespace {
                ext_attrs,
                partial,
                identifier,
                inheritance,
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Dictionary {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "dictionary")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) = preceded(
            parser::multispace_or_comment0,
            parser::parse_dictionary_members,
        )(input)?;

        Ok((
            input,
            Definition::Dictionary(Dictionary {
                ext_attrs,
                partial,
                identifier,
                inheritance,
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Enumeration {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, identifier) = parse_definition_identifier(input, "enum")?;
        let (input, values) = delimited(
            delimited(
                parser::multispace_or_comment0,
                tag("{"),
                parser::multispace_or_comment0,
            ),
            separated_list0(
                delimited(
                    parser::multispace_or_comment0,
                    tag(","),
                    parser::multispace_or_comment0,
                ),
                parser::parse_quoted_string,
            ),
            preceded(
                delimited(
                    parser::multispace_or_comment0,
                    // In case the last value has a comma at the end.
                    opt(tag(",")),
                    parser::multispace_or_comment0,
                ),
                tag("}"),
            ),
        )(input)?;

        Ok((
            input,
            Definition::Enumeration(Enumeration {
                ext_attrs,
                identifier,
                values: values.iter().map(|s| s.to_string()).collect(),
            }),
        ))
    }
}

impl Parser<Definition> for CallbackFunction {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, identifier) = parse_definition_identifier(input, "callback")?;
        let (input, r#type) = preceded(
            delimited(
                parser::multispace_or_comment1,
                tag("="),
                parser::multispace_or_comment0,
            ),
            Type::parse,
        )(input)?;
        let (input, arguments) = preceded(multispace0, parser::parse_arguments)(input)?;

        Ok((
            input,
            Definition::CallbackFunction(CallbackFunction {
                ext_attrs,
                identifier,
                r#type,
                arguments,
            }),
        ))
    }
}

impl Parser<Definition> for Typedef {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, r#type) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("typedef"),
                parser::multispace_or_comment1,
            ),
            Type::parse,
        )(input)?;
        let (input, identifier) = map(
            preceded(parser::multispace_or_comment1, parser::parse_identifier),
            |s| s.to_string(),
        )(input)?;

        Ok((
            input,
            Definition::Typedef(Typedef {
                ext_attrs,
                r#type,
                identifier,
            }),
        ))
    }
}
