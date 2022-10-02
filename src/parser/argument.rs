use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::delimited,
    IResult,
};

use crate::{Argument, ExtendedAttribute, Parser};

impl Parser<()> for Argument {
    fn parse(input: &str) -> IResult<&str, ()> {
        let (input, ext_attrs) = ExtendedAttribute::parse(input)?;
        let (input, optional) = map(
            opt(delimited(multispace0, tag("optional"), multispace1)),
            |o| o.is_some(),
        )(input)?;

        // TODO
        Ok((input, ()))
    }
}
