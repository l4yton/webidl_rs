use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    parser, CallbackInterface, Definition, ExtendedAttribute, Includes, Interface, InterfaceMixin,
    Member, Namespace, Parser,
};

impl Parser<Definition> for Definition {
    fn parse(input: &str) -> IResult<&str, Definition> {
        alt((
            Interface::parse,
            InterfaceMixin::parse,
            Includes::parse,
            CallbackInterface::parse,
        ))(input)
    }
}

impl Parser<Definition> for Interface {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, partial) = map(
            opt(delimited(multispace0, tag("partial"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(multispace0, tag("interface"), multispace1),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(multispace0, tag(":"), multispace0),
            parser::identifier,
        ))(input)?;
        let (input, members) = Member::parse(input)?;

        Ok((
            input,
            Definition::Interface(Interface {
                ext_attrs,
                partial,
                identifier: identifier.to_string(),
                inheritance: inheritance.map(|s| s.to_string()),
                members,
            }),
        ))
    }
}

impl Parser<Definition> for InterfaceMixin {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, partial) = map(
            opt(delimited(multispace0, tag("partial"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(multispace0, tag("interface mixin"), multispace1),
            parser::identifier,
        )(input)?;
        let (input, members) = Member::parse(input)?;

        Ok((
            input,
            Definition::InterfaceMixin(InterfaceMixin {
                ext_attrs,
                partial,
                identifier: identifier.to_string(),
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Includes {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, (interface, mixin)) = terminated(
            separated_pair(
                delimited(multispace0, parser::identifier, multispace1),
                tag("includes"),
                preceded(multispace1, parser::identifier),
            ),
            preceded(multispace0, tag(";")),
        )(input)?;

        Ok((
            input,
            Definition::Includes(Includes {
                ext_attrs,
                interface: interface.to_string(),
                mixin: mixin.to_string(),
            }),
        ))
    }
}

impl Parser<Definition> for CallbackInterface {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, identifier) = preceded(multispace0, parser::identifier)(input)?;
        let (input, members) = Member::parse(input)?;

        Ok((
            input,
            Definition::CallbackInterface(CallbackInterface {
                ext_attrs,
                identifier: identifier.to_string(),
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Namespace {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, partial) = map(
            opt(delimited(multispace0, tag("partial"), multispace1)),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(multispace0, tag("namespace"), multispace1),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(multispace0, tag(":"), multispace0),
            parser::identifier,
        ))(input)?;
        let (input, members) = Member::parse(input)?;

        Ok((
            input,
            Definition::Namespace(Namespace {
                ext_attrs,
                partial,
                identifier: identifier.to_string(),
                inheritance: inheritance.map(|s| s.to_string()),
                members,
            }),
        ))
    }
}
