use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res, not, opt, peek},
    multi::{many0, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    parser, Argument, CallbackFunction, CallbackInterface, DefaultValue, Definition, Dictionary,
    DictionaryMember, Enumeration, ExtAttrValue, ExtendedAttribute, Includes, Interface,
    InterfaceMixin, Member, NamedArgumentList, Namespace, Type, Typedef,
};

/// Checks if there is any inheritance specified (e.g.: `: Foo `). Returns the identifier of the
/// inherited definition.
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

/// Parses the following: `|name| <identifier>` and returns `<identifier>`.
fn parse_identifier_for_definition<'a>(input: &'a str, name: &str) -> IResult<&'a str, String> {
    preceded(
        delimited(
            parser::multispace_or_comment0,
            tag(name),
            parser::multispace_or_comment1,
        ),
        parser::parse_identifier,
    )(input)
}

impl Definition {
    pub fn parse(input: &str) -> IResult<&str, Definition> {
        alt((
            map(Interface::parse, Definition::Interface),
            map(InterfaceMixin::parse, Definition::InterfaceMixin),
            map(Includes::parse, Definition::Includes),
            map(CallbackInterface::parse, Definition::CallbackInterface),
            map(Namespace::parse, Definition::Namespace),
            map(Dictionary::parse, Definition::Dictionary),
            map(Enumeration::parse, Definition::Enumeration),
            map(CallbackFunction::parse, Definition::CallbackFunction),
            map(Typedef::parse, Definition::Typedef),
        ))(input)
    }
}

impl Interface {
    pub fn parse(input: &str) -> IResult<&str, Interface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_some_attribute(input, "partial")?;
        let (input, identifier) = parse_identifier_for_definition(input, "interface")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) = Member::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Interface {
                ext_attrs,
                partial,
                identifier,
                inheritance,
                members,
            },
        ))
    }
}

impl InterfaceMixin {
    pub fn parse(input: &str) -> IResult<&str, InterfaceMixin> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_some_attribute(input, "partial")?;
        let (input, identifier) = parse_identifier_for_definition(input, "interface mixin")?;
        let (input, members) = Member::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            InterfaceMixin {
                ext_attrs,
                partial,
                identifier,
                members,
            },
        ))
    }
}

impl Includes {
    pub fn parse(input: &str) -> IResult<&str, Includes> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, (interface, mixin)) = preceded(
            parser::multispace_or_comment0,
            separated_pair(
                parser::parse_identifier,
                delimited(
                    parser::multispace_or_comment1,
                    tag("includes"),
                    parser::multispace_or_comment1,
                ),
                parser::parse_identifier,
            ),
        )(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Includes {
                ext_attrs,
                interface,
                mixin,
            },
        ))
    }
}

impl CallbackInterface {
    pub fn parse(input: &str) -> IResult<&str, CallbackInterface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parse_identifier_for_definition(input, "callback interface")?;
        let (input, members) = Member::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            CallbackInterface {
                ext_attrs,
                identifier,
                members,
            },
        ))
    }
}

impl Namespace {
    pub fn parse(input: &str) -> IResult<&str, Namespace> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_some_attribute(input, "partial")?;
        let (input, identifier) = parse_identifier_for_definition(input, "namespace")?;
        let (input, members) = Member::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Namespace {
                ext_attrs,
                partial,
                identifier,
                members,
            },
        ))
    }
}

impl Dictionary {
    pub fn parse(input: &str) -> IResult<&str, Dictionary> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_some_attribute(input, "partial")?;
        let (input, identifier) = parse_identifier_for_definition(input, "dictionary")?;
        let (input, inheritance) = parse_optional_inheritance(input)?;
        let (input, members) = DictionaryMember::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Dictionary {
                ext_attrs,
                partial,
                identifier,
                inheritance,
                members,
            },
        ))
    }
}

impl Enumeration {
    pub fn parse(input: &str) -> IResult<&str, Enumeration> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parse_identifier_for_definition(input, "enum")?;
        let (input, values) = delimited(
            preceded(parser::multispace_or_comment0, tag("{")),
            separated_list0(
                tag(","),
                delimited(
                    parser::multispace_or_comment0,
                    parser::parse_quoted_string,
                    parser::multispace_or_comment0,
                ),
            ),
            delimited(
                // This is just in case the last value has a comma at the end.
                opt(preceded(parser::multispace_or_comment0, tag(","))),
                parser::multispace_or_comment0,
                tag("}"),
            ),
        )(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Enumeration {
                ext_attrs,
                identifier,
                values,
            },
        ))
    }
}

impl CallbackFunction {
    pub fn parse(input: &str) -> IResult<&str, CallbackFunction> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parse_identifier_for_definition(input, "callback")?;
        let (input, r#type) = preceded(
            preceded(parser::multispace_or_comment1, tag("=")),
            Type::parse,
        )(input)?;
        let (input, arguments) = Argument::parse_multi0(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            CallbackFunction {
                ext_attrs,
                identifier,
                r#type,
                arguments,
            },
        ))
    }
}

impl Typedef {
    pub fn parse(input: &str) -> IResult<&str, Typedef> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
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

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            Typedef {
                ext_attrs,
                r#type,
                identifier,
            },
        ))
    }
}

impl DictionaryMember {
    pub(crate) fn parse(input: &str) -> IResult<&str, DictionaryMember> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, required) = parser::parse_is_some_attribute(input, "required")?;
        let (input, r#type) = terminated(Type::parse, parser::multispace_or_comment1)(input)?;
        let (input, identifier) = parser::parse_identifier(input)?;
        let (input, default) = opt(preceded(
            preceded(parser::multispace_or_comment0, tag("=")),
            DefaultValue::parse,
        ))(input)?;

        let (input, _) = preceded(parser::multispace_or_comment0, tag(";"))(input)?;
        Ok((
            input,
            DictionaryMember {
                ext_attrs,
                required,
                r#type,
                identifier,
                default,
            },
        ))
    }

    pub(crate) fn parse_multi0(input: &str) -> IResult<&str, Vec<DictionaryMember>> {
        delimited(
            preceded(parser::multispace_or_comment0, tag("{")),
            many0(Self::parse),
            preceded(parser::multispace_or_comment0, tag("}")),
        )(input)
    }
}

impl ExtendedAttribute {
    pub(crate) fn parse(input: &str) -> IResult<&str, ExtendedAttribute> {
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::parse_identifier)(input)?;
        let (input, value) = opt(alt((
            preceded(
                preceded(parser::multispace_or_comment0, tag("=")),
                ExtAttrValue::parse,
            ),
            // This is deprecated, but was used by: `Constructor(double x, double y)`.
            // Although this isn't technically a value, we parse the arguments as such.
            map(Argument::parse_multi0, ExtAttrValue::ArgumentList),
        )))(input)?;

        Ok((input, ExtendedAttribute { identifier, value }))
    }

    pub(crate) fn parse_multi0(input: &str) -> IResult<&str, Vec<ExtendedAttribute>> {
        map(
            opt(delimited(
                preceded(parser::multispace_or_comment0, tag("[")),
                separated_list0(
                    tag(","),
                    terminated(Self::parse, parser::multispace_or_comment0),
                ),
                preceded(parser::multispace_or_comment0, tag("]")),
            )),
            |o| o.unwrap_or_default(),
        )(input)
    }
}

impl ExtAttrValue {
    pub(crate) fn parse(input: &str) -> IResult<&str, ExtAttrValue> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(NamedArgumentList::parse, ExtAttrValue::NamedArgumentList),
                map(parser::parse_identifier, ExtAttrValue::Identifier),
                map(Self::parse_identifier_list, ExtAttrValue::IdentifierList),
                map(tag("*"), |_| ExtAttrValue::Wildcard),
                map(parser::parse_quoted_string, ExtAttrValue::Identifier),
            )),
        )(input)
    }

    fn parse_identifier_list(input: &str) -> IResult<&str, Vec<String>> {
        delimited(
            preceded(parser::multispace_or_comment0, tag("(")),
            separated_list0(
                tag(","),
                delimited(
                    parser::multispace_or_comment0,
                    alt((parser::parse_identifier, parser::parse_quoted_string)),
                    parser::multispace_or_comment0,
                ),
            ),
            preceded(parser::multispace_or_comment0, tag(")")),
        )(input)
    }
}

impl NamedArgumentList {
    pub(crate) fn parse(input: &str) -> IResult<&str, NamedArgumentList> {
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::parse_identifier)(input)?;
        let (input, arguments) = Argument::parse_multi0(input)?;

        Ok((
            input,
            NamedArgumentList {
                identifier,
                arguments,
            },
        ))
    }
}

impl Argument {
    pub(crate) fn parse(input: &str) -> IResult<&str, Argument> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, optional) = parser::parse_is_some_attribute(input, "optional")?;
        let (input, r#type) = Type::parse(input)?;
        let (input, variadic) = parser::parse_is_some_attribute(input, "...")?;
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::parse_identifier)(input)?;
        let (input, default) = opt(preceded(
            preceded(parser::multispace_or_comment0, tag("=")),
            DefaultValue::parse,
        ))(input)?;

        Ok((
            input,
            Argument {
                ext_attrs,
                optional,
                r#type,
                variadic,
                identifier,
                default,
            },
        ))
    }

    pub(crate) fn parse_multi0(input: &str) -> IResult<&str, Vec<Argument>> {
        delimited(
            preceded(parser::multispace_or_comment0, tag("(")),
            separated_list0(
                tag(","),
                terminated(Self::parse, parser::multispace_or_comment0),
            ),
            preceded(parser::multispace_or_comment0, tag(")")),
        )(input)
    }
}

impl DefaultValue {
    pub(crate) fn parse(input: &str) -> IResult<&str, DefaultValue> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(alt((tag("true"), tag("false"))), |s: &str| {
                    DefaultValue::Boolean(s.parse::<bool>().unwrap())
                }),
                // Integer in hexadecimal format.
                map(preceded(tag("0x"), hex_digit1), |s: &str| {
                    DefaultValue::Integer(i64::from_str_radix(s, 16).unwrap())
                }),
                map(
                    // Make sure there is no "." at the end -> float
                    map_res(terminated(digit1, not(peek(tag(".")))), |s: &str| {
                        s.parse::<i64>()
                    }),
                    DefaultValue::Integer,
                ),
                // NOTE: Change this? Don't think we need f64 for WebIDL though.
                map(float, |f| DefaultValue::Decimal(f as f64)),
                map(
                    delimited(tag("\""), take_until("\""), tag("\"")),
                    |s: &str| DefaultValue::String(s.to_string()),
                ),
                map(tag("null"), |_| DefaultValue::Null),
                map(tag("Infinity"), |_| DefaultValue::Infinity),
                map(tag("-Infinity"), |_| DefaultValue::NegativeInfinity),
                map(tag("NaN"), |_| DefaultValue::NaN),
                map(tag("undefined"), |_| DefaultValue::Undefined),
                map(tag("[]"), |_| DefaultValue::Sequence),
                map(tag("{}"), |_| DefaultValue::Dictionary),
            )),
        )(input)
    }
}
