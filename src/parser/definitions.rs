use crate::input::WebIDLInput;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, hex_digit1},
    combinator::{map, map_res, not, opt, peek, value},
    multi::{many0, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use swc_atoms::JsWord;

use crate::{
    parser, Argument, CallbackFunction, CallbackInterface, DefaultValue, Definition, Dictionary,
    DictionaryMember, Enumeration, ExtAttrValue, ExtendedAttribute, Includes, Interface,
    InterfaceMixin, Member, NamedArgumentList, Namespace, Type, Typedef,
};

impl Definition {
    pub fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Definition> {
        terminated(
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
            )),
            tuple((parser::multispace_or_comment0, char(';'))),
        )(input)
    }
}

impl Interface {
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Interface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::is_partial(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "interface")?;
        let (input, inheritance) = parser::definition_inheritance(input)?;
        let (input, members) = Member::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, InterfaceMixin> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::is_partial(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "interface mixin")?;
        let (input, members) = Member::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Includes> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, (interface, mixin)) = preceded(
            parser::multispace_or_comment0,
            separated_pair(
                parser::idl_identifier,
                tuple((
                    parser::multispace_or_comment1,
                    tag("includes"),
                    parser::multispace_or_comment1,
                )),
                parser::idl_identifier,
            ),
        )(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, CallbackInterface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "callback interface")?;
        let (input, members) = Member::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Namespace> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::is_partial(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "namespace")?;
        let (input, members) = Member::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Dictionary> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::is_partial(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "dictionary")?;
        let (input, inheritance) = parser::definition_inheritance(input)?;
        let (input, members) = DictionaryMember::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Enumeration> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "enum")?;
        let (input, values) = delimited(
            tuple((parser::multispace_or_comment0, char('{'))),
            separated_list0(
                char(','),
                delimited(
                    parser::multispace_or_comment0,
                    parser::double_quoted_string,
                    parser::multispace_or_comment0,
                ),
            ),
            tuple((
                parser::multispace_or_comment0,
                // This is just in case the last value has a comma at the end.
                opt(tuple((char(','), parser::multispace_or_comment0))),
                char('}'),
            )),
        )(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, CallbackFunction> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::defintion_identifier(input, "callback")?;
        let (input, r#type) = preceded(
            tuple((parser::multispace_or_comment1, char('='))),
            Type::parse,
        )(input)?;
        let (input, arguments) = Argument::parse_multi0(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Typedef> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, r#type) = preceded(
            tuple((
                parser::multispace_or_comment0,
                tag("typedef"),
                parser::multispace_or_comment1,
            )),
            Type::parse,
        )(input)?;
        let (input, identifier) =
            preceded(parser::multispace_or_comment1, parser::idl_identifier)(input)?;

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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, DictionaryMember> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, required) = parser::is_required(input)?;
        let (input, r#type) = terminated(Type::parse, parser::multispace_or_comment1)(input)?;
        let (input, identifier) = parser::idl_identifier(input)?;
        let (input, default) = opt(preceded(
            tuple((parser::multispace_or_comment0, char('='))),
            DefaultValue::parse,
        ))(input)?;

        let (input, _) = tuple((parser::multispace_or_comment0, char(';')))(input)?;
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

    pub(crate) fn parse_multi0(
        input: WebIDLInput<&str>,
    ) -> IResult<WebIDLInput<&str>, Vec<DictionaryMember>> {
        delimited(
            preceded(parser::multispace_or_comment0, char('{')),
            many0(Self::parse),
            preceded(parser::multispace_or_comment0, char('}')),
        )(input)
    }
}

impl ExtendedAttribute {
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ExtendedAttribute> {
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::idl_identifier)(input)?;
        let (input, value) = opt(alt((
            preceded(
                tuple((parser::multispace_or_comment0, char('='))),
                ExtAttrValue::parse,
            ),
            // This is deprecated, but was used by: `Constructor(double x, double y)`.
            // Although this isn't technically a value, we parse the arguments as such.
            map(Argument::parse_multi0, ExtAttrValue::ArgumentList),
        )))(input)?;

        Ok((input, ExtendedAttribute { identifier, value }))
    }

    pub(crate) fn parse_multi0(
        input: WebIDLInput<&str>,
    ) -> IResult<WebIDLInput<&str>, Vec<ExtendedAttribute>> {
        map(
            opt(delimited(
                tuple((parser::multispace_or_comment0, char('['))),
                separated_list0(char(','), Self::parse),
                tuple((parser::multispace_or_comment0, char(']'))),
            )),
            Option::unwrap_or_default,
        )(input)
    }
}

impl ExtAttrValue {
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ExtAttrValue> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(NamedArgumentList::parse, ExtAttrValue::NamedArgumentList),
                map(
                    alt((parser::idl_identifier, parser::double_quoted_string)),
                    ExtAttrValue::Identifier,
                ),
                map(Self::parse_identifier_list, ExtAttrValue::IdentifierList),
                value(ExtAttrValue::Wildcard, char('*')),
            )),
        )(input)
    }

    fn parse_identifier_list(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Vec<JsWord>> {
        delimited(
            tuple((parser::multispace_or_comment0, char('('))),
            separated_list0(
                char(','),
                delimited(
                    parser::multispace_or_comment0,
                    alt((parser::idl_identifier, parser::double_quoted_string)),
                    parser::multispace_or_comment0,
                ),
            ),
            tuple((parser::multispace_or_comment0, char(')'))),
        )(input)
    }
}

impl NamedArgumentList {
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, NamedArgumentList> {
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::idl_identifier)(input)?;
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
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, Argument> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, optional) = parser::is_optional(input)?;
        let (input, r#type) = Type::parse(input)?;
        let (input, variadic) = parser::is_variadic(input)?;
        let (input, identifier) =
            preceded(parser::multispace_or_comment0, parser::idl_identifier)(input)?;
        let (input, default) = opt(preceded(
            tuple((parser::multispace_or_comment0, char('='))),
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

    pub(crate) fn parse_multi0(
        input: WebIDLInput<&str>,
    ) -> IResult<WebIDLInput<&str>, Vec<Argument>> {
        delimited(
            tuple((parser::multispace_or_comment0, char('('))),
            separated_list0(char(','), Self::parse),
            tuple((parser::multispace_or_comment0, char(')'))),
        )(input)
    }
}

impl DefaultValue {
    // TODO: There is room for improvements here...
    pub(crate) fn parse(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, DefaultValue> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(alt((tag("true"), tag("false"))), |s: WebIDLInput<&str>| {
                    DefaultValue::Boolean(s.input.parse::<bool>().unwrap())
                }),
                // Integer in hexadecimal format.
                map(preceded(tag("0x"), hex_digit1), |s: WebIDLInput<&str>| {
                    DefaultValue::Integer(i64::from_str_radix(s.input, 16).unwrap())
                }),
                map(
                    // Make sure there is no "." at the end -> float
                    map_res(
                        terminated(digit1, not(peek(char('.')))),
                        |s: WebIDLInput<&str>| s.input.parse::<i64>(),
                    ),
                    DefaultValue::Integer,
                ),
                // NOTE: Change this? Don't think we need f64 for WebIDL though.
                map(float, |f| DefaultValue::Decimal(f as f64)),
                map(parser::double_quoted_string, |s: JsWord| {
                    DefaultValue::String(s)
                }),
                value(DefaultValue::Null, tag("null")),
                value(DefaultValue::Infinity, tag("Infinity")),
                value(DefaultValue::NegativeInfinity, tag("-Infinity")),
                value(DefaultValue::NaN, tag("NaN")),
                value(DefaultValue::Undefined, tag("undefined")),
                value(DefaultValue::Sequence, tag("[]")),
                value(DefaultValue::Dictionary, tag("{}")),
            )),
        )(input)
    }
}
