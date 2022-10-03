use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res, not, opt, peek, recognize, success},
    multi::separated_list0,
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{Argument, DefaultValue, ExtendedAttribute, Parser, Type};

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

impl Parser<Vec<Argument>> for Argument {
    fn parse(input: &str) -> IResult<&str, Vec<Argument>> {
        delimited(
            delimited(multispace0, tag("("), multispace0),
            separated_list0(
                delimited(multispace0, tag(","), multispace0),
                parse_single_argument,
            ),
            preceded(multispace0, tag(")")),
        )(input)
    }
}

fn parse_single_argument(input: &str) -> IResult<&str, Argument> {
    let (input, ext_attrs) = map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
    let (input, optional) = map(
        opt(delimited(multispace0, tag("optional"), multispace1)),
        |o| o.is_some(),
    )(input)?;
    let (input, r#type) = preceded(multispace0, Type::parse)(input)?;
    let (input, variadic) = map(opt(tag("...")), |o| o.is_some())(input)?;
    let (input, identifier) = preceded(multispace1, identifier)(input)?;
    let (input, default) = opt(preceded(
        delimited(multispace0, tag("="), multispace0),
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
