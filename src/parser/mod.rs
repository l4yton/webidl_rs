mod argument;
mod ext_attr;
mod interface;
mod member;
mod r#type;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::complete::digit1,
    combinator::{map, map_res, not, peek, recognize, success},
    number::complete::float,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{DefaultValue, Parser};

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
            // NOTE: Change this? Don't think we need f64 for WebIDL though.
            map(float, |f| DefaultValue::Decimal(f as f64)),
            map(preceded(tag("\""), take_until("\"")), |s: &str| {
                DefaultValue::String(s.to_string())
            }),
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
