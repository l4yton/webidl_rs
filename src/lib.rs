/* Web IDL data structures */
mod definitions;
mod members;
mod types;

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
use nom::{combinator::all_consuming, error::Error, multi::many0, sequence::terminated, Err};

pub fn parse(input: &str) -> Result<Vec<Definition>, Err<Error<String>>> {
    let (_, definitions) = all_consuming(terminated(
        many0(Definition::parse),
        parser::multispace_or_comment0,
    ))(input)
    .map_err(|e| e.to_owned())?;

    Ok(definitions)
}

pub fn to_string(definitions: &[Definition]) -> String {
    join(definitions, "\n\n")
}
