use itertools::join;
use nom::{
    combinator::eof,
    error::Error,
    multi::many0,
    sequence::{preceded, terminated},
    Err,
};

use crate::{parser, Definition};

pub fn parse(input: &str) -> Result<Vec<Definition>, Err<Error<String>>> {
    Ok(terminated(
        many0(Definition::parse),
        preceded(parser::multispace_or_comment0, eof),
    )(input)
    .map_err(|e| e.to_owned())?
    .1)
}

pub fn to_string(definitions: &[Definition]) -> String {
    join(definitions, "\n\n")
}
