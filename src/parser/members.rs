use crate::{internal::String, WebIDLInput};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt, value, verify},
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{
    parser, Argument, AttrSpecial, Attribute, ConstValue, Constant, Constructor, ExtendedAttribute,
    Iterable, Maplike, Member, OpSpecial, Operation, Setlike, Stringifier, Type,
};

impl Member {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Member> {
        terminated(
            alt((
                map(Constant::parse, Self::Constant),
                map(Attribute::parse, Self::Attribute),
                map(Operation::parse, Self::Operation),
                map(Constructor::parse, Self::Constructor),
                map(Stringifier::parse, Self::Stringifier),
                map(Iterable::parse, Self::Iterable),
                map(Maplike::parse, Self::Maplike),
                map(Setlike::parse, Self::Setlike),
            )),
            tuple((parser::parse_multispace_or_comment0, char(';'))),
        )(input)
    }

    pub(crate) fn parse_multi0<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Vec<Member>> {
        delimited(
            tuple((parser::parse_multispace_or_comment0, char('{'))),
            many0(Self::parse),
            tuple((parser::parse_multispace_or_comment0, char('}'))),
        )(input)
    }
}

impl Constant {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Constant> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, (r#type, identifier)) = parser::parse_member_type_and_ident(input, "const")?;
        let (input, value) = preceded(
            tuple((parser::parse_multispace_or_comment0, char('='))),
            ConstValue::parse,
        )(input)?;

        Ok((
            input,
            Constant {
                ext_attrs,
                r#type,
                identifier,
                value,
            },
        ))
    }
}

impl ConstValue {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, ConstValue> {
        preceded(
            parser::parse_multispace_or_comment0,
            alt((
                map(parser::parse_bool, Self::Boolean),
                map(parser::parse_hex_number, Self::Integer),
                map(parser::parse_number, Self::Integer),
                map(double, Self::Decimal),
                value(Self::Infinity, tag("Infinity")),
                value(Self::NegativeInfinity, tag("-Infinity")),
                value(Self::NaN, tag("NaN")),
            )),
        )(input)
    }
}

impl Attribute {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Attribute> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, special) = AttrSpecial::parse(input)?;
        let (input, readonly) = parser::parse_is_readonly(input)?;
        let (input, (r#type, identifier)) =
            parser::parse_member_type_and_ident(input, "attribute")?;

        Ok((
            input,
            Attribute {
                ext_attrs,
                readonly,
                special,
                r#type,
                identifier,
            },
        ))
    }
}

impl AttrSpecial {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Option<AttrSpecial>> {
        opt(delimited(
            parser::parse_multispace_or_comment0,
            alt((
                value(Self::Static, tag("static")),
                value(Self::Stringifier, tag("stringifier")),
                value(Self::Inherit, tag("inherit")),
            )),
            parser::parse_multispace_or_comment1,
        ))(input)
    }
}

impl Operation {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Operation> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, special) = OpSpecial::parse(input)?;
        let (input, r#type) = Type::parse(input)?;
        let (input, identifier) = verify(
            map(
                opt(preceded(
                    parser::parse_multispace_or_comment1,
                    parser::parse_ident,
                )),
                Option::unwrap_or_default,
            ),
            |s: &String| !s.is_empty() || special.is_some(),
        )(input)?;
        let (input, arguments) = Argument::parse_multi0(input)?;

        Ok((
            input,
            Operation {
                ext_attrs,
                special,
                r#type,
                identifier,
                arguments,
            },
        ))
    }
}

impl OpSpecial {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Option<OpSpecial>> {
        opt(delimited(
            parser::parse_multispace_or_comment0,
            alt((
                value(Self::Getter, tag("getter")),
                value(Self::Setter, tag("setter")),
                value(Self::Deleter, tag("deleter")),
                value(Self::Static, tag("static")),
            )),
            parser::parse_multispace_or_comment1,
        ))(input)
    }
}

impl Constructor {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Constructor> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, arguments) = preceded(
            tuple((parser::parse_multispace_or_comment0, tag("constructor"))),
            Argument::parse_multi0,
        )(input)?;
        let r#type = Type::from(String::from(input.definition.unwrap()));

        Ok((
            input,
            Constructor {
                ext_attrs,
                arguments,
                r#type,
            },
        ))
    }
}

impl Stringifier {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Stringifier> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, _) = tuple((parser::parse_multispace_or_comment0, tag("stringifier")))(input)?;

        Ok((input, Stringifier { ext_attrs }))
    }
}

impl Iterable {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Iterable> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, r#async) = parser::parse_is_async(input)?;
        let (input, (first_type, second_type)) = delimited(
            tuple((
                parser::parse_multispace_or_comment0,
                tag("iterable"),
                parser::parse_multispace_or_comment0,
                char('<'),
            )),
            pair(
                Type::parse,
                opt(preceded(
                    tuple((parser::parse_multispace_or_comment0, char(','))),
                    Type::parse,
                )),
            ),
            tuple((parser::parse_multispace_or_comment0, char('>'))),
        )(input)?;
        let (input, arguments) = opt(Argument::parse_multi0)(input)?;

        // iterable<key, value>
        if let Some(value) = second_type {
            return Ok((
                input,
                Iterable {
                    ext_attrs,
                    r#async,
                    key: Some(first_type),
                    value,
                    arguments,
                },
            ));
        }

        // iterable<value>
        Ok((
            input,
            Iterable {
                ext_attrs,
                r#async,
                key: None,
                value: first_type,
                arguments,
            },
        ))
    }
}

impl Maplike {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Maplike> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, readonly) = parser::parse_is_readonly(input)?;
        let (input, (key, value)) = delimited(
            tuple((
                parser::parse_multispace_or_comment0,
                tag("maplike"),
                parser::parse_multispace_or_comment0,
                char('<'),
            )),
            separated_pair(
                Type::parse,
                tuple((parser::parse_multispace_or_comment0, char(','))),
                Type::parse,
            ),
            tuple((parser::parse_multispace_or_comment0, char('>'))),
        )(input)?;

        Ok((
            input,
            Maplike {
                ext_attrs,
                readonly,
                key,
                value,
            },
        ))
    }
}

impl Setlike {
    pub(crate) fn parse<'a>(
        input: WebIDLInput<'a, &'a str>,
    ) -> IResult<WebIDLInput<'a, &'a str>, Setlike> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, readonly) = parser::parse_is_readonly(input)?;
        let (input, r#type) = delimited(
            tuple((
                parser::parse_multispace_or_comment0,
                tag("setlike"),
                parser::parse_multispace_or_comment0,
                char('<'),
            )),
            Type::parse,
            tuple((parser::parse_multispace_or_comment0, char('>'))),
        )(input)?;

        Ok((
            input,
            Setlike {
                ext_attrs,
                readonly,
                r#type,
            },
        ))
    }
}
