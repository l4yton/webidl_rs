mod argument;
mod ext_attr;
mod interface;
mod member;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::{recognize, success},
    sequence::tuple,
    IResult,
};

// As definined in: https://webidl.spec.whatwg.org/#idl-grammar
pub(crate) fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        // [_-]?
        alt((tag("_"), tag("-"), success(""))),
        // [A-Za-z]
        take_while_m_n(1, 1, |s: char| s.is_ascii_alphabetic()),
        // [0-9A-Z_a-z-]*
        take_while(|s: char| s.is_ascii_alphanumeric() || s == '_' || s == '-'),
    )))(input)
}
