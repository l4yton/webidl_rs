use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::{delimited, preceded},
    IResult,
};

use crate::{parser, Argument, ExtendedAttribute, Parser, Type};

impl Parser<Argument> for Argument {
    fn parse(input: &str) -> IResult<&str, Argument> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or(vec![]))(input)?;
        let (input, optional) = map(
            opt(delimited(multispace0, tag("optional"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, r#type) = preceded(multispace0, Type::parse)(input)?;
        let (input, variadic) = map(opt(tag("...")), |o| o.is_some())(input)?;
        let (input, identifier) = preceded(multispace1, parser::identifier)(input)?;
        // TODO: DefaultValue

        Ok((
            input,
            Argument {
                ext_attrs,
                optional,
                r#type,
                variadic,
                identifier: identifier.to_string(),
                default: None,
            },
        ))
    }
}
