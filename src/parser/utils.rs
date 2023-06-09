use std::sync::OnceLock;

use crate::{input::WebIDLInput, Type};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace1},
    combinator::{map, opt, value},
    error::{Error, ErrorKind},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, tuple},
    Err, IResult, Slice,
};
use regex::Regex;
use swc_atoms::JsWord;

pub static IDL_IDENTIFIER_RE: OnceLock<Regex> = OnceLock::new();

pub(crate) fn idl_identifier_regex() -> Regex {
    Regex::new(r"^[_-]?[A-Za-z][0-9A-Z_a-z-]*").unwrap()
}

// As definined in: https://webidl.spec.whatwg.org/#idl-grammar
pub(crate) fn idl_identifier(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, JsWord> {
    if let Some(mat) = IDL_IDENTIFIER_RE
        .get_or_init(idl_identifier_regex)
        .find(input.input)
    {
        return Ok((input.slice(mat.end()..), JsWord::from(mat.as_str())));
    }

    Err(Err::Error(Error::new(input, ErrorKind::RegexpMatches)))
}

pub(crate) fn defintion_identifier<'a>(
    input: WebIDLInput<&'a str>,
    definition_tag: &str,
) -> IResult<WebIDLInput<&'a str>, JsWord> {
    let (mut input, identifier) = preceded(
        tuple((
            multispace_or_comment0,
            tag(definition_tag),
            multispace_or_comment1,
        )),
        idl_identifier,
    )(input)?;
    input.curr_definition = Some(identifier.clone());

    Ok((input, identifier))
}

pub(crate) fn definition_inheritance(
    input: WebIDLInput<&str>,
) -> IResult<WebIDLInput<&str>, Option<JsWord>> {
    opt(preceded(
        tuple((multispace_or_comment0, char(':'), multispace_or_comment0)),
        idl_identifier,
    ))(input)
}

pub(crate) fn member_type_and_identifier<'a>(
    input: WebIDLInput<&'a str>,
    member_tag: &str,
) -> IResult<WebIDLInput<&'a str>, (Type, JsWord)> {
    preceded(
        tuple((
            multispace_or_comment0,
            tag(member_tag),
            multispace_or_comment1,
        )),
        separated_pair(Type::parse, multispace_or_comment1, idl_identifier),
    )(input)
}

pub(crate) fn type_parameterized<'a>(
    input: WebIDLInput<&'a str>,
    type_tag: &str,
) -> IResult<WebIDLInput<&'a str>, Type> {
    delimited(
        tuple((
            multispace_or_comment0,
            tag(type_tag),
            multispace_or_comment0,
            char('<'),
        )),
        Type::parse,
        tuple((multispace_or_comment0, char('>'))),
    )(input)
}

fn single_line_comment(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ()> {
    value((), tuple((tag("//"), take_until("\n"), char('\n'))))(input)
}

fn multi_line_comment(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

pub(crate) fn multispace_or_comment0(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            single_line_comment,
            multi_line_comment,
        ))),
    )(input)
}

pub(crate) fn multispace_or_comment1(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, ()> {
    value(
        (),
        many1(alt((
            value((), multispace1),
            single_line_comment,
            multi_line_comment,
        ))),
    )(input)
}

pub(crate) fn double_quoted_string(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, JsWord> {
    map(
        delimited(char('"'), take_until("\""), char('"')),
        |s: WebIDLInput<_>| JsWord::from(s.input),
    )(input)
}

fn is_some_attribute<'a>(
    input: WebIDLInput<&'a str>,
    attribute: &str,
) -> IResult<WebIDLInput<&'a str>, bool> {
    map(
        opt(tuple((
            multispace_or_comment0,
            tag(attribute),
            multispace_or_comment1,
        ))),
        |o| o.is_some(),
    )(input)
}

pub(crate) fn is_partial(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "partial")
}

pub(crate) fn is_required(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "required")
}

pub(crate) fn is_optional(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "optional")
}

pub(crate) fn is_variadic(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "...")
}

pub(crate) fn is_readonly(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "readonly")
}

pub(crate) fn is_async(input: WebIDLInput<&str>) -> IResult<WebIDLInput<&str>, bool> {
    is_some_attribute(input, "async")
}
