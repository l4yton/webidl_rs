use nom::{
    bytes::complete::tag,
    combinator::{cond, eof},
    error::Error,
    multi::separated_list0,
    sequence::{delimited, preceded},
    Err,
};

use crate::{parser, Definition};

pub fn parse(input: &str) -> Result<Vec<Definition>, Err<Error<String>>> {
    // Making the error owned, makes it easier to use this function without having to deal with
    // potential lifetime issues.
    _parse(input).map_err(|e| e.to_owned())
}

fn _parse(input: &str) -> Result<Vec<Definition>, Err<Error<&str>>> {
    let (input, _) = parser::multispace_or_comment0(input)?;
    let (input, definitions) = separated_list0(
        delimited(
            parser::multispace_or_comment0,
            tag(";"),
            parser::multispace_or_comment0,
        ),
        Definition::parse,
    )(input)?;
    // `seperated_list0()` doesn't consume the last seperator, hence make sure that the last
    // definition also ends with a semicolon.
    let (input, _) = cond(
        !definitions.is_empty(),
        preceded(parser::multispace_or_comment0, tag(";")),
    )(input)?;
    let (_input, _) = preceded(parser::multispace_or_comment0, eof)(input)?;

    Ok(definitions)
}

pub fn to_string(definitions: &[Definition]) -> String {
    definitions
        .iter()
        .fold(String::new(), |mut string, definition| {
            string.push_str(&definition.to_string());
            string.push('\n');
            string.push('\n');
            string
        })
}
