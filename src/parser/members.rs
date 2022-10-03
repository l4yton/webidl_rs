use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res, not, opt, peek},
    multi::separated_list0,
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    parser, Argument, AttrSpecial, Attribute, ConstValue, Constant, Constructor, ExtendedAttribute,
    Iterable, Maplike, Member, OpSpecial, Operation, Parser, Setlike, Stringifer, Type,
};

impl Parser<Vec<Member>> for Member {
    fn parse(input: &str) -> IResult<&str, Vec<Member>> {
        terminated(
            delimited(
                delimited(multispace0, tag("{"), multispace0),
                terminated(
                    separated_list0(
                        delimited(multispace0, tag(";"), multispace0),
                        parse_single_member,
                    ),
                    preceded(multispace0, tag(";")),
                ),
                delimited(multispace0, tag("}"), multispace0),
            ),
            tag(";"),
        )(input)
    }
}

fn parse_single_member(input: &str) -> IResult<&str, Member> {
    alt((
        Constant::parse,
        Attribute::parse,
        Operation::parse,
        Constructor::parse,
        Stringifer::parse,
        Iterable::parse,
        Maplike::parse,
        Setlike::parse,
    ))(input)
}

impl Parser<Member> for Constant {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, r#type) = preceded(
            delimited(multispace0, tag("const"), multispace1),
            Type::parse,
        )(input)?;
        let (input, identifier) = preceded(multispace1, parser::identifier)(input)?;
        let (input, value) = preceded(
            delimited(multispace0, tag("="), multispace0),
            ConstValue::parse,
        )(input)?;

        Ok((
            input,
            Member::Constant(Constant {
                ext_attrs,
                r#type,
                identifier: identifier.to_string(),
                value,
            }),
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

impl Parser<Member> for Attribute {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, readonly) = map(
            opt(delimited(multispace0, tag("readonly"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, special) = opt(delimited(
            multispace0,
            alt((
                map(tag("static"), |_| AttrSpecial::Static),
                map(tag("stringifier"), |_| AttrSpecial::Stringifier),
                map(tag("inherit"), |_| AttrSpecial::Inherit),
            )),
            multispace1,
        ))(input)?;
        let (input, r#type) = preceded(
            delimited(multispace0, tag("attribute"), multispace1),
            Type::parse,
        )(input)?;
        let (input, identifier) = preceded(multispace1, parser::identifier)(input)?;

        Ok((
            input,
            Member::Attribute(Attribute {
                ext_attrs,
                readonly,
                special,
                r#type,
                identifier: identifier.to_string(),
            }),
        ))
    }
}

impl Parser<Member> for Operation {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, special) = opt(delimited(
            multispace0,
            alt((
                map(tag("static"), |_| OpSpecial::Static),
                map(tag("getter"), |_| OpSpecial::Getter),
                map(tag("setter"), |_| OpSpecial::Setter),
                map(tag("deleter"), |_| OpSpecial::Deleter),
            )),
            multispace1,
        ))(input)?;
        let (input, r#type) = delimited(multispace0, Type::parse, multispace1)(input)?;
        let (input, identifier) = parser::identifier(input)?;
        let (input, arguments) = Argument::parse(input)?;

        Ok((
            input,
            Member::Operation(Operation {
                ext_attrs,
                special,
                r#type,
                identifier: identifier.to_string(),
                arguments,
            }),
        ))
    }
}

impl Parser<Member> for Constructor {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, arguments) = preceded(tag("constructor"), Argument::parse)(input)?;

        Ok((
            input,
            Member::Constructor(Constructor {
                ext_attrs,
                arguments,
            }),
        ))
    }
}

impl Parser<Member> for Stringifer {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, _) = tag("stringifier")(input)?;

        Ok((input, Member::Stringifer(Stringifer { ext_attrs })))
    }
}

impl Parser<Member> for Iterable {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, r#async) = map(
            opt(delimited(multispace0, tag("async"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, (value_type, key_type)) = delimited(
            preceded(multispace0, tag("iterable<")),
            pair(
                Type::parse,
                opt(preceded(
                    delimited(multispace0, tag(","), multispace0),
                    Type::parse,
                )),
            ),
            tag(">"),
        )(input)?;
        let (input, arguments) = opt(Argument::parse)(input)?;

        // iterable<key_type, value_type>
        if let Some(key_type) = key_type {
            return Ok((
                input,
                Member::Iterable(Iterable {
                    ext_attrs,
                    r#async,
                    key_type: Some(value_type),
                    value_type: key_type,
                    arguments,
                }),
            ));
        }

        Ok((
            input,
            Member::Iterable(Iterable {
                ext_attrs,
                r#async,
                key_type,
                value_type,
                arguments,
            }),
        ))
    }
}

impl Parser<Member> for Maplike {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, readonly) = map(
            opt(delimited(multispace0, tag("readonly"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, (key_type, value_type)) = delimited(
            preceded(multispace0, tag("maplike<")),
            separated_pair(
                Type::parse,
                delimited(multispace0, tag(","), multispace0),
                Type::parse,
            ),
            tag(">"),
        )(input)?;

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

impl Parser<Member> for Setlike {
    fn parse(input: &str) -> IResult<&str, Member> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, readonly) = map(
            opt(delimited(multispace0, tag("readonly"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, r#type) = delimited(
            preceded(multispace0, tag("setlike<")),
            Type::parse,
            tag(">"),
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
