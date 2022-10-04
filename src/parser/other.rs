use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::complete::{digit1, hex_digit1, multispace1},
    combinator::{map, map_res, not, opt, peek, recognize, success},
    multi::{many0, many1, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{Argument, DefaultValue, DictionaryMember, ExtendedAttribute, Parser, Type};

// As definined in: https://webidl.spec.whatwg.org/#idl-grammar
pub(crate) fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        // [_-]?
        alt((tag("_"), tag("-"), success(""))),
        // [A-Za-z]
        take_while_m_n(1, 1, |s: char| s.is_ascii_alphabetic()),
        // [0-9A-Z_a-z-]*
        take_while(|s: char| s.is_ascii_alphanumeric() || s == '_' || s == '-'),
    )))(input)
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

impl Parser<Vec<Argument>> for Argument {
    fn parse(input: &str) -> IResult<&str, Vec<Argument>> {
        delimited(
            terminated(tag("("), multispace_or_comment0),
            separated_list0(
                delimited(multispace_or_comment0, tag(","), multispace_or_comment0),
                parse_single_argument,
            ),
            preceded(multispace_or_comment0, tag(")")),
        )(input)
    }
}

fn parse_single_argument(input: &str) -> IResult<&str, Argument> {
    let (input, ext_attrs) = map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
    let (input, optional) = map(
        opt(delimited(
            multispace_or_comment0,
            tag("optional"),
            multispace_or_comment1,
        )),
        |o| o.is_some(),
    )(input)?;
    let (input, r#type) = preceded(multispace_or_comment0, Type::parse)(input)?;
    let (input, variadic) = map(opt(tag("...")), |o| o.is_some())(input)?;
    let (input, identifier) = preceded(multispace_or_comment1, identifier)(input)?;
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
            identifier: identifier.to_string(),
            default,
        },
    ))
}

impl Parser<DefaultValue> for DefaultValue {
    fn parse(input: &str) -> IResult<&str, DefaultValue> {
        alt((
            map(alt((tag("true"), tag("false"))), |s: &str| {
                DefaultValue::Boolean(s.parse::<bool>().unwrap())
            }),
            map(
                // Make sure there is no "." at the end -> float
                map_res(terminated(digit1, not(peek(tag(".")))), |s: &str| {
                    s.parse::<i64>()
                }),
                DefaultValue::Integer,
            ),
            // Integer in hexadecimal format.
            map(preceded(tag("0x"), hex_digit1), |s: &str| {
                DefaultValue::Integer(i64::from_str_radix(s, 16).unwrap())
            }),
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

impl Parser<Vec<DictionaryMember>> for DictionaryMember {
    fn parse(input: &str) -> IResult<&str, Vec<DictionaryMember>> {
        delimited(
            terminated(tag("{"), multispace_or_comment0),
            terminated(
                separated_list0(
                    delimited(multispace_or_comment0, tag(";"), multispace_or_comment0),
                    parse_single_dictionary_member,
                ),
                preceded(multispace_or_comment0, tag(";")),
            ),
            delimited(multispace_or_comment0, tag("}"), multispace_or_comment0),
        )(input)
    }
}

fn parse_single_dictionary_member(input: &str) -> IResult<&str, DictionaryMember> {
    let (input, ext_attrs) = map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
    let (input, required) = map(
        opt(delimited(
            multispace_or_comment0,
            tag("required"),
            multispace_or_comment1,
        )),
        |o| o.is_some(),
    )(input)?;
    let (input, r#type) = preceded(multispace_or_comment0, Type::parse)(input)?;
    let (input, identifier) = preceded(multispace_or_comment1, identifier)(input)?;
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
            identifier: identifier.to_string(),
            default,
        },
    ))
}
