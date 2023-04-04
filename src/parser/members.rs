use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, hex_digit1},
    combinator::{map, map_res, not, opt, peek, value},
    multi::many0,
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use swc_atoms::JsWord;

use crate::{
    parser, Argument, AttrSpecial, Attribute, ConstValue, Constant, Constructor, ExtendedAttribute,
    Iterable, Maplike, Member, OpSpecial, Operation, Setlike, Stringifer, Type,
};

fn parse_type_and_identifier_for_member<'a>(
    input: &'a str,
    name: &str,
) -> IResult<&'a str, (Type, JsWord)> {
    preceded(
        tuple((
            parser::multispace_or_comment0,
            tag(name),
            parser::multispace_or_comment1,
        )),
        separated_pair(
            Type::parse,
            parser::multispace_or_comment1,
            parser::parse_identifier,
        ),
    )(input)
}

impl Member {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        terminated(
            alt((
                map(Constant::parse, Member::Constant),
                map(Attribute::parse, Member::Attribute),
                map(Constructor::parse, Member::Constructor),
                map(Operation::parse, Member::Operation),
                map(Stringifer::parse, Member::Stringifer),
                map(Iterable::parse, Member::Iterable),
                map(Maplike::parse, Member::Maplike),
                map(Setlike::parse, Member::Setlike),
            )),
            tuple((parser::multispace_or_comment0, char(';'))),
        )(input)
    }

    pub(crate) fn parse_multi0(input: &str) -> IResult<&str, Vec<Member>> {
        delimited(
            tuple((parser::multispace_or_comment0, char('{'))),
            many0(Self::parse),
            tuple((parser::multispace_or_comment0, char('}'))),
        )(input)
    }
}

impl Constant {
    pub(crate) fn parse(input: &str) -> IResult<&str, Constant> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, (r#type, identifier)) = parse_type_and_identifier_for_member(input, "const")?;
        let (input, value) = preceded(
            tuple((parser::multispace_or_comment0, char('='))),
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
    pub(crate) fn parse(input: &str) -> IResult<&str, ConstValue> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(alt((tag("true"), tag("false"))), |s: &str| {
                    ConstValue::Boolean(s.parse::<bool>().unwrap())
                }),
                // Integer in hexadecimal format.
                map(preceded(tag("0x"), hex_digit1), |s: &str| {
                    ConstValue::Integer(i64::from_str_radix(s, 16).unwrap())
                }),
                map(
                    // Make sure there is no "." at the end -> float.
                    map_res(terminated(digit1, not(peek(char('.')))), |s: &str| {
                        s.parse::<i64>()
                    }),
                    ConstValue::Integer,
                ),
                // NOTE: Change this? Don't think we need f64 for WebIDL though.
                map(float, |f| ConstValue::Decimal(f as f64)),
                value(ConstValue::Infinity, tag("Infinity")),
                value(ConstValue::NegativeInfinity, tag("-Infinity")),
                value(ConstValue::NaN, tag("NaN")),
            )),
        )(input)
    }
}

impl Attribute {
    pub(crate) fn parse(input: &str) -> IResult<&str, Attribute> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, special) = AttrSpecial::parse(input)?;
        let (input, readonly) = parser::parse_is_some_attribute(input, "readonly")?;
        let (input, (r#type, identifier)) =
            parse_type_and_identifier_for_member(input, "attribute")?;

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
    pub(crate) fn parse(input: &str) -> IResult<&str, Option<AttrSpecial>> {
        opt(delimited(
            parser::multispace_or_comment0,
            alt((
                value(AttrSpecial::Static, tag("static")),
                value(AttrSpecial::Stringifier, tag("stringifier")),
                value(AttrSpecial::Inherit, tag("inherit")),
            )),
            parser::multispace_or_comment1,
        ))(input)
    }
}

impl Operation {
    pub(crate) fn parse(input: &str) -> IResult<&str, Operation> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, special) = OpSpecial::parse(input)?;
        // Can't use `parse_member_type()` here since an operation doesn't have a tag and
        // therefore also no space or comment between tag and type.
        let (input, r#type) = terminated(
            Type::parse,
            // Special operations may have the arguments directly after the type and thus no
            // space afterwards, for example:
            // `getter CSSNumericValue(unsigned long index)`
            if special.is_some() {
                parser::multispace_or_comment0
            } else {
                parser::multispace_or_comment1
            },
        )(input)?;
        let (input, identifier) =
            map(opt(parser::parse_identifier), |o| o.unwrap_or_default())(input)?;
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
    pub(crate) fn parse(input: &str) -> IResult<&str, Option<OpSpecial>> {
        opt(delimited(
            parser::multispace_or_comment0,
            alt((
                value(OpSpecial::Static, tag("static")),
                value(OpSpecial::Getter, tag("getter")),
                value(OpSpecial::Setter, tag("setter")),
                value(OpSpecial::Deleter, tag("deleter")),
            )),
            parser::multispace_or_comment1,
        ))(input)
    }
}

impl Constructor {
    pub(crate) fn parse(input: &str) -> IResult<&str, Constructor> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, arguments) = preceded(
            tuple((parser::multispace_or_comment0, tag("constructor"))),
            Argument::parse_multi0,
        )(input)?;

        Ok((
            input,
            Constructor {
                ext_attrs,
                arguments,
            },
        ))
    }
}

impl Stringifer {
    pub(crate) fn parse(input: &str) -> IResult<&str, Stringifer> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, _) = tuple((parser::multispace_or_comment0, tag("stringifier")))(input)?;

        Ok((input, Stringifer { ext_attrs }))
    }
}

impl Iterable {
    pub(crate) fn parse(input: &str) -> IResult<&str, Iterable> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, r#async) = parser::parse_is_some_attribute(input, "async")?;
        let (input, (first_type, second_type)) = delimited(
            tuple((
                parser::multispace_or_comment0,
                tag("iterable"),
                parser::multispace_or_comment0,
                char('<'),
            )),
            pair(
                Type::parse,
                opt(preceded(
                    tuple((parser::multispace_or_comment0, char(','))),
                    Type::parse,
                )),
            ),
            tuple((parser::multispace_or_comment0, char('>'))),
        )(input)?;
        let (input, arguments) = opt(Argument::parse_multi0)(input)?;

        // iterable<key_type, value_type>
        if let Some(value_type) = second_type {
            return Ok((
                input,
                Iterable {
                    ext_attrs,
                    r#async,
                    key_type: Some(first_type),
                    value_type,
                    arguments,
                },
            ));
        }

        // iterable<value_type>
        Ok((
            input,
            Iterable {
                ext_attrs,
                r#async,
                key_type: None,
                value_type: first_type,
                arguments,
            },
        ))
    }
}

impl Maplike {
    pub(crate) fn parse(input: &str) -> IResult<&str, Maplike> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, readonly) = parser::parse_is_some_attribute(input, "readonly")?;
        let (input, (key_type, value_type)) = delimited(
            tuple((
                parser::multispace_or_comment0,
                tag("maplike"),
                parser::multispace_or_comment0,
                char('<'),
            )),
            separated_pair(
                Type::parse,
                tuple((parser::multispace_or_comment0, char(','))),
                Type::parse,
            ),
            tuple((parser::multispace_or_comment0, char('>'))),
        )(input)?;

        Ok((
            input,
            Maplike {
                ext_attrs,
                readonly,
                key_type,
                value_type,
            },
        ))
    }
}

impl Setlike {
    pub(crate) fn parse(input: &str) -> IResult<&str, Setlike> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, readonly) = parser::parse_is_some_attribute(input, "readonly")?;
        let (input, r#type) = delimited(
            tuple((
                parser::multispace_or_comment0,
                tag("setlike"),
                parser::multispace_or_comment0,
                char('<'),
            )),
            Type::parse,
            tuple((parser::multispace_or_comment0, char('>'))),
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
