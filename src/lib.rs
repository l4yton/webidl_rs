mod definitions;
mod display;
mod input;
mod members;
mod parser;
mod types;

#[cfg(test)]
mod tests;

pub use definitions::*;
pub use input::WebIDLInput;
pub use members::*;
pub use types::*;

pub fn parse(
    input: &str,
) -> Result<Vec<Definition>, nom::Err<nom::error::Error<WebIDLInput<&str>>>> {
    let (_, definitions) = nom::combinator::all_consuming(nom::sequence::terminated(
        nom::multi::many0(Definition::parse),
        parser::multispace_or_comment0,
    ))(WebIDLInput::from(input))?;

    Ok(definitions)
}

pub fn to_string(definitions: &[Definition]) -> String {
    itertools::join(definitions, "\n\n")
}
