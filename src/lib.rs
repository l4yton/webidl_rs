/* Web IDL data structures */
mod consts;
mod definitions;
mod members;
mod types;

pub use consts::*;
pub use definitions::*;
pub use members::*;
pub use types::*;

/* Display and parser logic */
mod display;
mod parser;

/* Tests */
#[cfg(test)]
mod tests;

/* Exposed functions */
use itertools::join;
use nom::{
    combinator::eof,
    error::Error,
    multi::many0,
    sequence::{preceded, terminated},
    Err,
};

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
