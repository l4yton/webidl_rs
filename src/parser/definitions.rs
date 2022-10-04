use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{
    parser, Argument, CallbackFunction, CallbackInterface, Definition, Dictionary,
    DictionaryMember, Enumeration, ExtendedAttribute, Includes, Interface, InterfaceMixin, Member,
    Namespace, Parser, Type, Typedef,
};

impl Parser<Definition> for Definition {
    fn parse(input: &str) -> IResult<&str, Definition> {
        alt((
            Interface::parse,
            InterfaceMixin::parse,
            Includes::parse,
            CallbackInterface::parse,
            Namespace::parse,
            Dictionary::parse,
            Enumeration::parse,
            CallbackFunction::parse,
            Typedef::parse,
        ))(input)
    }
}

impl Parser<Definition> for Interface {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, partial) = map(
            opt(delimited(
                parser::multispace_or_comment0,
                tag("partial"),
                parser::multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("interface"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(":"),
                parser::multispace_or_comment0,
            ),
            parser::identifier,
        ))(input)?;
        let (input, members) = preceded(parser::multispace_or_comment0, Member::parse)(input)?;

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
            opt(delimited(
                parser::multispace_or_comment0,
                tag("partial"),
                parser::multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("interface mixin"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, members) = preceded(parser::multispace_or_comment0, Member::parse)(input)?;

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
        let (input, (interface, mixin)) = separated_pair(
            delimited(
                parser::multispace_or_comment0,
                parser::identifier,
                parser::multispace_or_comment1,
            ),
            tag("includes"),
            preceded(parser::multispace_or_comment1, parser::identifier),
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
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("callback interface"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, members) = preceded(parser::multispace_or_comment0, Member::parse)(input)?;

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
            opt(delimited(
                parser::multispace_or_comment0,
                tag("partial"),
                parser::multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("namespace"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(":"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        ))(input)?;
        let (input, members) = preceded(parser::multispace_or_comment0, Member::parse)(input)?;

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

impl Parser<Definition> for Dictionary {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, partial) = map(
            opt(delimited(
                parser::multispace_or_comment0,
                tag("partial"),
                parser::multispace_or_comment1,
            )),
            |o| o.is_some(),
        )(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("dictionary"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, inheritance) = opt(preceded(
            delimited(
                parser::multispace_or_comment0,
                tag(":"),
                parser::multispace_or_comment0,
            ),
            parser::identifier,
        ))(input)?;
        let (input, members) =
            preceded(parser::multispace_or_comment0, DictionaryMember::parse)(input)?;

        Ok((
            input,
            Definition::Dictionary(Dictionary {
                ext_attrs,
                partial,
                identifier: identifier.to_string(),
                inheritance: inheritance.map(|s| s.to_string()),
                members,
            }),
        ))
    }
}

impl Parser<Definition> for Enumeration {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("enum"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, values) = delimited(
            delimited(
                parser::multispace_or_comment0,
                tag("{"),
                parser::multispace_or_comment0,
            ),
            separated_list0(
                delimited(
                    parser::multispace_or_comment0,
                    tag(","),
                    parser::multispace_or_comment0,
                ),
                delimited(tag("\""), take_until("\""), tag("\"")),
            ),
            delimited(
                parser::multispace_or_comment0,
                tag("}"),
                parser::multispace_or_comment0,
            ),
        )(input)?;

        Ok((
            input,
            Definition::Enumeration(Enumeration {
                ext_attrs,
                identifier: identifier.to_string(),
                values: values.iter().map(|s| s.to_string()).collect(),
            }),
        ))
    }
}

impl Parser<Definition> for CallbackFunction {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, identifier) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("callback"),
                parser::multispace_or_comment1,
            ),
            parser::identifier,
        )(input)?;
        let (input, r#type) = preceded(
            delimited(
                parser::multispace_or_comment1,
                tag("="),
                parser::multispace_or_comment0,
            ),
            Type::parse,
        )(input)?;
        let (input, arguments) = preceded(multispace0, Argument::parse)(input)?;

        Ok((
            input,
            Definition::CallbackFunction(CallbackFunction {
                ext_attrs,
                identifier: identifier.to_string(),
                r#type,
                arguments,
            }),
        ))
    }
}

impl Parser<Definition> for Typedef {
    fn parse(input: &str) -> IResult<&str, Definition> {
        let (input, ext_attrs) =
            map(opt(ExtendedAttribute::parse), |o| o.unwrap_or_default())(input)?;
        let (input, r#type) = preceded(
            delimited(
                parser::multispace_or_comment0,
                tag("typedef"),
                parser::multispace_or_comment1,
            ),
            Type::parse,
        )(input)?;
        let (input, identifier) =
            preceded(parser::multispace_or_comment1, parser::identifier)(input)?;

        Ok((
            input,
            Definition::Typedef(Typedef {
                ext_attrs,
                r#type,
                identifier: identifier.to_string(),
            }),
        ))
    }
}
