use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::complete::multispace1,
    combinator::{map, opt, recognize, success},
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};
use swc_atoms::JsWord;

// As definined in: https://webidl.spec.whatwg.org/#idl-grammar
pub(crate) fn parse_identifier(input: &str) -> IResult<&str, JsWord> {
    map(
        recognize(tuple((
            // [_-]?
            alt((tag("_"), tag("-"), success(""))),
            // [A-Za-z]
            take_while_m_n(1, 1, |s: char| s.is_ascii_alphabetic()),
            // [0-9A-Z_a-z-]*
            take_while(|s: char| s.is_ascii_alphanumeric() || s == '_' || s == '-'),
        ))),
        |s| JsWord::from(s),
    )(input)
}

pub(crate) fn multispace_or_comment0(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((
        multispace1,
        delimited(tag("//"), take_until("\n"), tag("\n")),
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )))(input)
}

pub(crate) fn multispace_or_comment1(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((
        multispace1,
        delimited(tag("//"), take_until("\n"), tag("\n")),
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )))(input)
}

pub(crate) fn parse_quoted_string(input: &str) -> IResult<&str, JsWord> {
    map(
        delimited(tag("\""), take_until("\""), tag("\"")),
        |s: &str| JsWord::from(s),
    )(input)
}

pub(crate) fn parse_is_some_attribute<'a>(
    input: &'a str,
    attribute: &str,
) -> IResult<&'a str, bool> {
    map(
        opt(tuple((
            multispace_or_comment0,
            tag(attribute),
            multispace_or_comment1,
        ))),
        |o| o.is_some(),
    )(input)
}
