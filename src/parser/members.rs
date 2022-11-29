use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res, not, opt, peek},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    parser, AttrSpecial, Attribute, ConstValue, Constant, Constructor, Iterable, Maplike, Member,
    OpSpecial, Operation, Setlike, Stringifer, Type,
};

fn parse_member_type<'a>(input: &'a str, member_tag: &str) -> IResult<&'a str, Type> {
    terminated(
        preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(member_tag),
                parser::multispace_or_comment1,
            ),
            Type::parse,
        ),
        parser::multispace_or_comment1,
    )(input)
}

fn parse_check_is_readonly(input: &str) -> IResult<&str, bool> {
    map(
        opt(delimited(
            parser::multispace_or_comment0,
            tag("readonly"),
            parser::multispace_or_comment1,
        )),
        |o| o.is_some(),
    )(input)
}

impl Member {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        alt((
            Constant::parse,
            Attribute::parse,
            Constructor::parse,
            Operation::parse,
            Stringifer::parse,
            Iterable::parse,
            Maplike::parse,
            Setlike::parse,
        ))(input)
    }
}

impl Constant {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, r#type) = parse_member_type(input, "const")?;
        let (input, identifier) = parser::parse_identifier(input)?;
        let (input, value) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("="),
                parser::multispace_or_comment0,
            ),
            ConstValue::parse,
        )(input)?;

        Ok((
            input,
            Member::Constant(Constant {
                ext_attrs,
                r#type,
                identifier,
                value,
            }),
        ))
    }
}

impl ConstValue {
    pub(crate) fn parse(input: &str) -> IResult<&str, ConstValue> {
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
                map_res(terminated(digit1, not(peek(tag(".")))), |s: &str| {
                    s.parse::<i64>()
                }),
                ConstValue::Integer,
            ),
            // NOTE: Change this? Don't think we need f64 for WebIDL though.
            map(float, |f| ConstValue::Decimal(f as f64)),
            map(tag("Infinity"), |_| ConstValue::Infinity),
            map(tag("-Infinity"), |_| ConstValue::NegativeInfinity),
            map(tag("NaN"), |_| ConstValue::NaN),
        ))(input)
    }
}

impl Attribute {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, special) = opt(delimited(
            parser::multispace_or_comment0,
            alt((
                map(tag("static"), |_| AttrSpecial::Static),
                map(tag("stringifier"), |_| AttrSpecial::Stringifier),
                map(tag("inherit"), |_| AttrSpecial::Inherit),
            )),
            parser::multispace_or_comment1,
        ))(input)?;
        let (input, readonly) = parse_check_is_readonly(input)?;
        let (input, r#type) = parse_member_type(input, "attribute")?;
        let (input, identifier) = parser::parse_identifier(input)?;

        Ok((
            input,
            Member::Attribute(Attribute {
                ext_attrs,
                readonly,
                special,
                r#type,
                identifier,
            }),
        ))
    }
}

impl Operation {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, special) = opt(delimited(
            parser::multispace_or_comment0,
            alt((
                map(tag("static"), |_| OpSpecial::Static),
                map(tag("getter"), |_| OpSpecial::Getter),
                map(tag("setter"), |_| OpSpecial::Setter),
                map(tag("deleter"), |_| OpSpecial::Deleter),
            )),
            parser::multispace_or_comment1,
        ))(input)?;
        // Can't use `parse_member_type()` here since an operation doesn't have a tag and
        // therefore also no space or comment between tag and type.
        let (input, r#type) = delimited(
            parser::multispace_or_comment0,
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
        let (input, arguments) =
            preceded(parser::multispace_or_comment0, parser::parse_arguments)(input)?;

        // TODO: return error instead.
        assert!(
            !identifier.is_empty() || special.is_some(),
            "Found regular operation with no identifier"
        );

        Ok((
            input,
            Member::Operation(Operation {
                ext_attrs,
                special,
                r#type,
                identifier,
                arguments,
            }),
        ))
    }
}

impl Constructor {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, _) = preceded(parser::multispace_or_comment0, tag("constructor"))(input)?;
        let (input, arguments) =
            preceded(parser::multispace_or_comment0, parser::parse_arguments)(input)?;

        Ok((
            input,
            Member::Constructor(Constructor {
                ext_attrs,
                arguments,
            }),
        ))
    }
}

impl Stringifer {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, _) = preceded(parser::multispace_or_comment0, tag("stringifier"))(input)?;

        Ok((input, Member::Stringifer(Stringifer { ext_attrs })))
    }
}

impl Iterable {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, r#async) = map(
            opt(delimited(
                parser::multispace_or_comment0,
                tag("async"),
                parser::multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("iterable"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("<"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, first_type) = Type::parse(input)?;
        let (input, second_type) = opt(preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(","),
                parser::multispace_or_comment0,
            ),
            Type::parse,
        ))(input)?;
        let (input, _) = preceded(parser::multispace_or_comment0, tag(">"))(input)?;
        let (input, arguments) = opt(preceded(
            parser::multispace_or_comment0,
            parser::parse_arguments,
        ))(input)?;

        // iterable<key_type, value_type>
        if let Some(value_type) = second_type {
            return Ok((
                input,
                Member::Iterable(Iterable {
                    ext_attrs,
                    r#async,
                    key_type: Some(first_type),
                    value_type,
                    arguments,
                }),
            ));
        }

        Ok((
            input,
            Member::Iterable(Iterable {
                ext_attrs,
                r#async,
                key_type: None,
                value_type: first_type,
                arguments,
            }),
        ))
    }
}

impl Maplike {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, readonly) = parse_check_is_readonly(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("maplike"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("<"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, (key_type, value_type)) = separated_pair(
            Type::parse,
            delimited(
                parser::multispace_or_comment0,
                tag(","),
                parser::multispace_or_comment0,
            ),
            Type::parse,
        )(input)?;
        let (input, _) = preceded(parser::multispace_or_comment0, tag(">"))(input)?;

        Ok((
            input,
            Member::Maplike(Maplike {
                ext_attrs,
                readonly,
                key_type,
                value_type,
            }),
        ))
    }
}

impl Setlike {
    pub(crate) fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, readonly) = parse_check_is_readonly(input)?;
        let (input, _) = delimited(
            parser::multispace_or_comment0,
            tag("setlike"),
            parser::multispace_or_comment0,
        )(input)?;
        let (input, r#type) = delimited(
            delimited(
                parser::multispace_or_comment0,
                tag("<"),
                parser::multispace_or_comment0,
            ),
            Type::parse,
            preceded(parser::multispace_or_comment0, tag(">")),
        )(input)?;

        Ok((
            input,
            Member::Setlike(Setlike {
                ext_attrs,
                readonly,
                r#type,
            }),
        ))
    }
}
