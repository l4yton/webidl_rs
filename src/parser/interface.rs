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
        let (input, ext_attrs) = ExtendedAttribute::parse(input)?;
        let (input, partial) = map(
            opt(delimited(multispace0, tag("partial"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            terminated(tag("interface"), multispace1),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(delimited(
            delimited(multispace0, tag(":"), multispace0),
            parser::identifier,
            multispace0,
        ))(input)?;
        let (input, members) = terminated(
            delimited(
                tag("{"),
                separated_list0(delimited(multispace0, tag(";"), multispace0), Member::parse),
                tag("}"),
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
