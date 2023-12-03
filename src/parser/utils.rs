use std::sync::OnceLock;

use crate::{internal::String, Type, WebIDLInput};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, hex_digit1, multispace1},
    combinator::{map, map_res, not, opt, peek, value},
    error::{Error, ErrorKind},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Err, IResult, Slice,
};
use regex::Regex;

pub static IDL_IDENTIFIER_RE: OnceLock<Regex> = OnceLock::new();

/// As definined in: <https://webidl.spec.whatwg.org/#idl-grammar>
pub(crate) fn idl_identifier_regex() -> Regex {
    Regex::new(r"^[_-]?[A-Za-z][0-9A-Z_a-z-]*").unwrap()
}

pub(crate) fn parse_ident_str<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, &'a str> {
    if let Some(mat) = IDL_IDENTIFIER_RE
        .get_or_init(idl_identifier_regex)
        .find(input.input)
    {
        return Ok((input.slice(mat.end()..), mat.as_str()));
    }

    Err(Err::Error(Error::new(input, ErrorKind::RegexpMatches)))
}

pub(crate) fn parse_ident<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, String> {
    map(parse_ident_str, String::from)(input)
}

pub(crate) fn parse_def_ident<'a>(
    input: WebIDLInput<'a, &'a str>,
    definition_tag: &str,
) -> IResult<WebIDLInput<'a, &'a str>, String> {
    let (mut input, identifier) = preceded(
        tuple((
            parse_multispace_or_comment0,
            tag(definition_tag),
            parse_multispace_or_comment1,
        )),
        parse_ident_str,
    )(input)?;
    input.definition = Some(identifier);

    Ok((input, String::from(identifier)))
}

pub(crate) fn parse_def_inheritance<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, Option<String>> {
    opt(preceded(
        tuple((
            parse_multispace_or_comment0,
            char(':'),
            parse_multispace_or_comment0,
        )),
        parse_ident,
    ))(input)
}

pub(crate) fn parse_member_type_and_ident<'a>(
    input: WebIDLInput<'a, &'a str>,
    member_tag: &str,
) -> IResult<WebIDLInput<'a, &'a str>, (Type, String)> {
    preceded(
        tuple((
            parse_multispace_or_comment0,
            tag(member_tag),
            parse_multispace_or_comment1,
        )),
        separated_pair(Type::parse, parse_multispace_or_comment1, parse_ident),
    )(input)
}

pub(crate) fn parse_parameterized_type<'a>(
    input: WebIDLInput<'a, &'a str>,
    type_tag: &str,
) -> IResult<WebIDLInput<'a, &'a str>, Box<Type>> {
    delimited(
        tuple((
            parse_multispace_or_comment0,
            tag(type_tag),
            parse_multispace_or_comment0,
            char('<'),
        )),
        map(Type::parse, Box::new),
        tuple((parse_multispace_or_comment0, char('>'))),
    )(input)
}

fn parse_single_line_comment<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, ()> {
    value((), tuple((tag("//"), take_until("\n"), char('\n'))))(input)
}

fn parse_multi_line_comment<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

pub(crate) fn parse_multispace_or_comment0<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, ()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            parse_single_line_comment,
            parse_multi_line_comment,
        ))),
    )(input)
}

pub(crate) fn parse_multispace_or_comment1<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, ()> {
    value(
        (),
        many1(alt((
            value((), multispace1),
            parse_single_line_comment,
            parse_multi_line_comment,
        ))),
    )(input)
}

pub(crate) fn parse_double_quoted_string<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, String> {
    map(
        delimited(char('"'), take_until("\""), char('"')),
        |s: WebIDLInput<_>| String::from(s.input),
    )(input)
}

fn parse_is_some_attribute<'a>(
    input: WebIDLInput<'a, &'a str>,
    attribute: &str,
) -> IResult<WebIDLInput<'a, &'a str>, bool> {
    map(
        opt(tuple((
            parse_multispace_or_comment0,
            tag(attribute),
            parse_multispace_or_comment1,
        ))),
        |o| o.is_some(),
    )(input)
}

macro_rules! parse_is_some_fn {
    ($fn_name: ident, $attribute: expr) => {
        pub(crate) fn $fn_name<'a>(
            input: WebIDLInput<'a, &'a str>,
        ) -> IResult<WebIDLInput<'a, &'a str>, bool> {
            parse_is_some_attribute(input, $attribute)
        }
    };
}

parse_is_some_fn!(parse_is_partial, "partial");
parse_is_some_fn!(parse_is_required, "required");
parse_is_some_fn!(parse_is_optional, "optional");
parse_is_some_fn!(parse_is_variadic, "...");
parse_is_some_fn!(parse_is_readonly, "readonly");
parse_is_some_fn!(parse_is_async, "async");

pub(crate) fn parse_bool<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, bool> {
    map_res(alt((tag("true"), tag("false"))), |s: WebIDLInput<&str>| {
        s.input.parse()
    })(input)
}

pub(crate) fn parse_hex_number<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, i64> {
    map_res(preceded(tag("0x"), hex_digit1), |s: WebIDLInput<&str>| {
        i64::from_str_radix(s.input, 16)
    })(input)
}

pub(crate) fn parse_number<'a>(
    input: WebIDLInput<'a, &'a str>,
) -> IResult<WebIDLInput<'a, &'a str>, i64> {
    map_res(
        terminated(digit1, not(peek(char('.')))),
        |s: WebIDLInput<&str>| s.input.parse(),
    )(input)
}
