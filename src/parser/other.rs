use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::complete::{digit1, hex_digit1, multispace1},
    combinator::{map, map_res, not, opt, peek, recognize, success},
    error::Error,
    multi::{many0, many1, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{Argument, DefaultValue, DictionaryMember, ExtendedAttribute, Member, Type};

// As definined in: https://webidl.spec.whatwg.org/#idl-grammar
pub(crate) fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((
            // [_-]?
            alt((tag("_"), tag("-"), success(""))),
            // [A-Za-z]
            take_while_m_n(1, 1, |s: char| s.is_ascii_alphabetic()),
            // [0-9A-Z_a-z-]*
            take_while(|s: char| s.is_ascii_alphanumeric() || s == '_' || s == '-'),
        ))),
        |s| s.to_string(),
    )(input)
}

pub(crate) fn multispace_or_comment0(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((
        multispace1,
        delimited(tag("//"), take_until("\n"), tag("\n")),
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )))(input)
}

pub(crate) fn multispace_or_comment1(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((
        multispace1,
        delimited(tag("//"), take_until("\n"), tag("\n")),
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )))(input)
}

pub(crate) fn parse_ext_attrs(input: &str) -> IResult<&str, Vec<ExtendedAttribute>> {
    map(
        opt(delimited(
            terminated(tag("["), multispace_or_comment0),
            separated_list0(
                delimited(multispace_or_comment0, tag(","), multispace_or_comment0),
                ExtendedAttribute::parse,
            ),
            preceded(multispace_or_comment0, tag("]")),
        )),
        |o| o.unwrap_or_default(),
    )(input)
}

// NOTE: Currently this only accepts strings in double quotes. Should this also parse strings in
//       single quotes?
pub(crate) fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            tag::<&str, &str, Error<&str>>("\""),
            take_until("\""),
            tag("\""),
        ),
        |s| s.to_string(),
    )(input)
}

pub(crate) fn parse_dictionary_members(input: &str) -> IResult<&str, Vec<DictionaryMember>> {
    delimited(
        terminated(tag("{"), multispace_or_comment0),
        terminated(
            separated_list0(
                delimited(multispace_or_comment0, tag(";"), multispace_or_comment0),
                DictionaryMember::parse,
            ),
            // Make the tag(";") optional in case there are no member.
            preceded(multispace_or_comment0, opt(tag(";"))),
        ),
        delimited(multispace_or_comment0, tag("}"), multispace_or_comment0),
    )(input)
}

pub(crate) fn parse_arguments(input: &str) -> IResult<&str, Vec<Argument>> {
    delimited(
        terminated(tag("("), multispace_or_comment0),
        separated_list0(
            delimited(multispace_or_comment0, tag(","), multispace_or_comment0),
            Argument::parse,
        ),
        preceded(multispace_or_comment0, tag(")")),
    )(input)
}

pub(crate) fn parse_members(input: &str) -> IResult<&str, Vec<Member>> {
    delimited(
        terminated(tag("{"), multispace_or_comment0),
        terminated(
            separated_list0(
                delimited(multispace_or_comment0, tag(";"), multispace_or_comment0),
                Member::parse,
            ),
            // Make the tag(";") optional in case there are no member.
            preceded(multispace_or_comment0, opt(tag(";"))),
        ),
        preceded(multispace_or_comment0, tag("}")),
    )(input)
}

impl Argument {
    pub(crate) fn parse(input: &str) -> IResult<&str, Argument> {
        let (input, ext_attrs) = parse_ext_attrs(input)?;
        let (input, optional) = map(
            opt(delimited(
                multispace_or_comment0,
                tag("optional"),
                multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, r#type) = preceded(multispace_or_comment0, Type::parse)(input)?;
        let (input, variadic) = map(opt(preceded(multispace_or_comment0, tag("..."))), |o| {
            o.is_some()
        })(input)?;
        let (input, identifier) = preceded(multispace_or_comment1, parse_identifier)(input)?;
        let (input, default) = opt(preceded(
            delimited(multispace_or_comment0, tag("="), multispace_or_comment0),
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
}

impl DefaultValue {
    pub(crate) fn parse(input: &str) -> IResult<&str, DefaultValue> {
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
        ))(input)
    }
}

impl DictionaryMember {
    pub(crate) fn parse(input: &str) -> IResult<&str, DictionaryMember> {
        let (input, ext_attrs) = parse_ext_attrs(input)?;
        let (input, required) = map(
            opt(delimited(
                multispace_or_comment0,
                tag("required"),
                multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, r#type) = preceded(multispace_or_comment0, Type::parse)(input)?;
        let (input, identifier) = preceded(multispace_or_comment1, parse_identifier)(input)?;
        let (input, default) = opt(preceded(
            delimited(multispace_or_comment0, tag("="), multispace_or_comment0),
            DefaultValue::parse,
        ))(input)?;

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
}
