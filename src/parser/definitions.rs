use crate::{internal::String, WebIDLInput};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt, value},
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{
    parser, Argument, CallbackFunction, CallbackInterface, DefaultValue, Definition, Dictionary,
    DictionaryMember, Enumeration, ExtAttrValue, ExtendedAttribute, Includes, Interface,
    InterfaceMixin, Member, NamedArgumentList, Namespace, Type, Typedef,
};

impl Definition {
    pub fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Definition> {
        terminated(
            alt((
                map(Interface::parse, Self::Interface),
                map(InterfaceMixin::parse, Self::InterfaceMixin),
                map(Includes::parse, Self::Includes),
                map(CallbackInterface::parse, Self::CallbackInterface),
                map(Namespace::parse, Self::Namespace),
                map(Dictionary::parse, Self::Dictionary),
                map(Enumeration::parse, Self::Enumeration),
                map(CallbackFunction::parse, Self::CallbackFunction),
                map(Typedef::parse, Self::Typedef),
            )),
            tuple((parser::parse_multispace_or_comment0, char(';'))),
        )(input)
    }
}

impl Interface {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Interface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_partial(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "interface")?;
        let (input, inheritance) = parser::parse_def_inheritance(input)?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, InterfaceMixin> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_partial(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "interface mixin")?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Includes> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, (interface, mixin)) = preceded(
            parser::parse_multispace_or_comment0,
            separated_pair(
                parser::parse_ident,
                tuple((
                    parser::parse_multispace_or_comment1,
                    tag("includes"),
                    parser::parse_multispace_or_comment1,
                )),
                parser::parse_ident,
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, CallbackInterface> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "callback interface")?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Namespace> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_partial(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "namespace")?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Dictionary> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, partial) = parser::parse_is_partial(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "dictionary")?;
        let (input, inheritance) = parser::parse_def_inheritance(input)?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Enumeration> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "enum")?;
        let (input, values) = delimited(
            tuple((parser::parse_multispace_or_comment0, char('{'))),
            separated_list0(
                char(','),
                delimited(
                    parser::parse_multispace_or_comment0,
                    parser::parse_double_quoted_string,
                    parser::parse_multispace_or_comment0,
                ),
            ),
            tuple((
                parser::parse_multispace_or_comment0,
                // This is just in case the last value has a comma at the end.
                opt(tuple((char(','), parser::parse_multispace_or_comment0))),
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, CallbackFunction> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, identifier) = parser::parse_def_ident(input, "callback")?;
        let (input, r#type) = preceded(
            tuple((parser::parse_multispace_or_comment1, char('='))),
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Typedef> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, r#type) = preceded(
            tuple((
                parser::parse_multispace_or_comment0,
                tag("typedef"),
                parser::parse_multispace_or_comment1,
            )),
            Type::parse,
        )(input)?;
        let (input, identifier) =
            preceded(parser::parse_multispace_or_comment1, parser::parse_ident)(input)?;

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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, DictionaryMember> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, required) = parser::parse_is_required(input)?;
        let (input, r#type) = terminated(Type::parse, parser::parse_multispace_or_comment1)(input)?;
        let (input, identifier) = parser::parse_ident(input)?;
        let (input, default) = opt(preceded(
            tuple((parser::parse_multispace_or_comment0, char('='))),
            DefaultValue::parse,
        ))(input)?;

        let (input, _) = tuple((parser::parse_multispace_or_comment0, char(';')))(input)?;
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

    pub(crate) fn parse_multi0<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Vec<DictionaryMember>> {
        delimited(
            preceded(parser::parse_multispace_or_comment0, char('{')),
            many0(Self::parse),
            preceded(parser::parse_multispace_or_comment0, char('}')),
        )(input)
    }
}

impl ExtendedAttribute {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, ExtendedAttribute> {
        let (input, identifier) =
            preceded(parser::parse_multispace_or_comment0, parser::parse_ident)(input)?;
        let (input, value) = opt(alt((
            preceded(
                tuple((parser::parse_multispace_or_comment0, char('='))),
                ExtAttrValue::parse,
            ),
            // This is deprecated, but was used by: `Constructor(double x, double y)`.
            // Although this isn't technically a value, we parse the arguments as such.
            map(Argument::parse_multi0, ExtAttrValue::ArgumentList),
        )))(input)?;

        Ok((input, ExtendedAttribute { identifier, value }))
    }

    pub(crate) fn parse_multi0<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Vec<ExtendedAttribute>> {
        map(
            opt(delimited(
                tuple((parser::parse_multispace_or_comment0, char('['))),
                separated_list0(char(','), Self::parse),
                tuple((parser::parse_multispace_or_comment0, char(']'))),
            )),
            Option::unwrap_or_default,
        )(input)
    }
}

impl ExtAttrValue {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, ExtAttrValue> {
        preceded(
            parser::parse_multispace_or_comment0,
            alt((
                map(NamedArgumentList::parse, Self::NamedArgumentList),
                map(
                    alt((parser::parse_ident, parser::parse_double_quoted_string)),
                    Self::Identifier,
                ),
                map(Self::parse_identifier_list, Self::IdentifierList),
                value(Self::Wildcard, char('*')),
            )),
        )(input)
    }

    fn parse_identifier_list<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Vec<String>> {
        delimited(
            tuple((parser::parse_multispace_or_comment0, char('('))),
            separated_list0(
                char(','),
                delimited(
                    parser::parse_multispace_or_comment0,
                    alt((parser::parse_ident, parser::parse_double_quoted_string)),
                    parser::parse_multispace_or_comment0,
                ),
            ),
            tuple((parser::parse_multispace_or_comment0, char(')'))),
        )(input)
    }
}

impl NamedArgumentList {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, NamedArgumentList> {
        let (input, identifier) =
            preceded(parser::parse_multispace_or_comment0, parser::parse_ident)(input)?;
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
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Argument> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, optional) = parser::parse_is_optional(input)?;
        let (input, r#type) = Type::parse(input)?;
        let (input, variadic) = parser::parse_is_variadic(input)?;
        let (input, identifier) =
            preceded(parser::parse_multispace_or_comment0, parser::parse_ident)(input)?;
        let (input, default) = opt(preceded(
            tuple((parser::parse_multispace_or_comment0, char('='))),
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

    pub(crate) fn parse_multi0<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Vec<Argument>> {
        delimited(
            tuple((parser::parse_multispace_or_comment0, char('('))),
            separated_list0(char(','), Self::parse),
            tuple((parser::parse_multispace_or_comment0, char(')'))),
        )(input)
    }
}

impl DefaultValue {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, DefaultValue> {
        preceded(
            parser::parse_multispace_or_comment0,
            alt((
                map(parser::parse_bool, Self::Boolean),
                map(parser::parse_hex_number, Self::Integer),
                map(parser::parse_number, Self::Integer),
                map(double, Self::Decimal),
                map(parser::parse_double_quoted_string, Self::String),
                value(Self::Null, tag("null")),
                value(Self::Infinity, tag("Infinity")),
                value(Self::NegativeInfinity, tag("-Infinity")),
                value(Self::NaN, tag("NaN")),
                value(Self::Undefined, tag("undefined")),
                value(Self::Sequence, tag("[]")),
                value(Self::Dictionary, tag("{}")),
            )),
        )(input)
    }
}
