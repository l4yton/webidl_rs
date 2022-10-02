use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::{parser, ExtendedAttribute, Interface, Member, Parser};

impl Parser<Interface> for Interface {
    fn parse(input: &str) -> IResult<&str, Interface> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or(vec![]))(input)?;
        let (input, partial) = map(
            opt(delimited(multispace0, tag("partial"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            terminated(tag("interface"), multispace1),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(multispace0, tag(":"), multispace0),
            parser::identifier,
        ))(input)?;
        let (input, members) = terminated(
            delimited(
                preceded(multispace0, tag("{")),
                separated_list0(delimited(multispace0, tag(";"), multispace0), Member::parse),
                preceded(multispace0, tag("}")),
            ),
            tag(";"),
        )(input)?;

        Ok((
            input,
            Self {
                ext_attrs,
                partial,
                identifier: identifier.to_string(),
                inheritance: inheritance.map(|s| s.to_string()),
                members,
            },
        ))
    }
}
