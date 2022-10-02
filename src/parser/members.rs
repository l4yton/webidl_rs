use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res, not, opt, peek},
    number::complete::float,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{parser, ConstValue, Constant, ExtendedAttribute, Member, Parser, Type};

impl Parser<Member> for Member {
    fn parse(input: &str) -> IResult<&str, Member> {
        // TODO
        Ok((
            input,
            Member::Stringifer(crate::Stringifer { ext_attrs: vec![] }),
        ))
    }
}

impl Parser<Constant> for Constant {
    fn parse(input: &str) -> IResult<&str, Constant> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, r#type) = preceded(
            delimited(multispace0, tag("const"), multispace1),
            Type::parse,
        )(input)?;
        let (input, identifier) = parser::identifier(input)?;
        let (input, value) = preceded(
            delimited(multispace0, tag("="), multispace0),
            ConstValue::parse,
        )(input)?;

        Ok((
            input,
            Constant {
                ext_attrs,
                r#type,
                identifier: identifier.to_string(),
                value,
            },
        ))
    }
}

impl Parser<ConstValue> for ConstValue {
    fn parse(input: &str) -> IResult<&str, ConstValue> {
        alt((
            map(alt((tag("true"), tag("false"))), |s: &str| {
                ConstValue::Boolean(s.parse::<bool>().unwrap())
            }),
            map(
                // Make sure there is no "." at the end -> float
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
