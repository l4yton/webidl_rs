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
    Interface, InterfaceMixin, Member, Namespace, Type, Typedef,
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
    opt(preceded(
        delimited(
            parser::multispace_or_comment0,
            tag(":"),
            parser::multispace_or_comment0,
        ),
        parser::parse_identifier,
    ))(input)
}

fn parse_definition_identifier<'a>(
    input: &'a str,
    definition_tag: &str,
) -> IResult<&'a str, String> {
    preceded(
        delimited(
            parser::multispace_or_comment0,
            tag(definition_tag),
            parser::multispace_or_comment1,
        ),
        parser::parse_identifier,
    )(input)
}

impl Definition {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
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

impl Interface {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "interface")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        // "A partial interface definition cannot specify that the interface inherits from another interface.
        // Inheritance is to be specified on the original interface definition"
        assert!(
            !partial || inheritance.is_none(),
            "A partial interface shall not specify inheritance"
        );

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

impl InterfaceMixin {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
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

impl Includes {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, _) = parser::multispace_or_comment0(input)?;
        let (input, (interface, mixin)) = separated_pair(
            parser::parse_identifier,
            delimited(
                parser::multispace_or_comment1,
                tag("includes"),
                parser::multispace_or_comment1,
            ),
            parser::parse_identifier,
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

impl CallbackInterface {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, identifier) = parse_definition_identifier(input, "callback interface")?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        // "Callback interfaces must define exactly one regular operation."
        assert!(
            members
                .iter()
                .filter(|member| matches!(member, Member::Operation(_)))
                .count()
                == 1
        );

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

impl Namespace {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "namespace")?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, parser::parse_members)(input)?;

        Ok((
            input,
            Definition::Namespace(Namespace {
                ext_attrs,
                partial,
                identifier,
                members,
            }),
        ))
    }
}

impl Dictionary {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, partial) = parse_check_is_partial(input)?;
        let (input, identifier) = parse_definition_identifier(input, "dictionary")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) = preceded(
            parser::multispace_or_comment0,
            parser::parse_dictionary_members,
        )(input)?;

        // Same as with interfaces, partial dictionaries should not specify inheritance.
        assert!(
            !partial || inheritance.is_none(),
            "A partial dictionary shall not specify inheritance"
        );

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

impl Enumeration {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, identifier) = parse_definition_identifier(input, "enum")?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("{"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, values) = separated_list0(
            delimited(
                parser::multispace_or_comment0,
                tag(","),
                parser::multispace_or_comment0,
            ),
            parser::parse_quoted_string,
        )(input)?;
        // In case the last value has a comma at the end.
        let (input, _) = opt(delimited(
            parser::multispace_or_comment0,
            tag(","),
            parser::multispace_or_comment0,
        ))(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("}"),
            parser::multispace_or_comment0,
        )(input)?;

        Ok((
            input,
            Definition::Enumeration(Enumeration {
                ext_attrs,
                identifier,
                values,
            }),
        ))
    }
}

impl CallbackFunction {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
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

impl Typedef {
    pub(crate) fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, r#type) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("typedef"),
                parser::multispace_or_comment1,
            ),
            Type::parse,
        )(input)?;
        let (input, identifier) =
            preceded(parser::multispace_or_comment1, parser::parse_identifier)(input)?;

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
