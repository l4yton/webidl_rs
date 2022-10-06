use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{parser, Parser, RecordType, StandardType, StandardTypeName, Type, UnionType};

fn parse_parameterized_type<'a>(input: &'a str, name: &str) -> IResult<&'a str, Type> {
    delimited(
        delimited(tag(name), parser::multispace_or_comment0, tag("<")),
        delimited(
            parser::multispace_or_comment0,
            Type::parse,
            parser::multispace_or_comment0,
        ),
        tag(">"),
    )(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = parse_parameterized_type(input, "sequence")?;
    Ok((input, Type::Sequence(Box::new(r#type))))
}

fn parse_promise(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = parse_parameterized_type(input, "Promise")?;
    Ok((input, Type::Promise(Box::new(r#type))))
}

fn parse_frozen_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = parse_parameterized_type(input, "FrozenArray")?;
    Ok((input, Type::FrozenArray(Box::new(r#type))))
}

fn parse_observable_array(input: &str) -> IResult<&str, Type> {
    let (input, r#type) = parse_parameterized_type(input, "ObservableArray")?;
    Ok((input, Type::ObservableArray(Box::new(r#type))))
}

fn parse_record(input: &str) -> IResult<&str, Type> {
    let (input, (key, value)) = delimited(
        delimited(tag("record"), parser::multispace_or_comment0, tag(">")),
        separated_pair(
            delimited(
                parser::multispace_or_comment0,
                Type::parse,
                parser::multispace_or_comment0,
            ),
            tag(","),
            delimited(
                parser::multispace_or_comment0,
                Type::parse,
                parser::multispace_or_comment0,
            ),
        ),
        tag(">"),
    )(input)?;

    Ok((
        input,
        Type::Record(RecordType {
            key: Box::new(key),
            value: Box::new(value),
        }),
    ))
}
impl Parser<Type> for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        alt((
            parse_sequence,
            parse_record,
            parse_promise,
            parse_frozen_array,
            parse_observable_array,
            map(UnionType::parse, Type::Union),
            map(StandardType::parse, Type::Standard),
        ))(input)
    }
}

impl Parser<UnionType> for UnionType {
    fn parse(input: &str) -> IResult<&str, UnionType> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, types) = delimited(
            delimited(
                parser::multispace_or_comment0,
                tag("("),
                parser::multispace_or_comment0,
            ),
            separated_list1(
                delimited(
                    parser::multispace_or_comment1,
                    tag("or"),
                    parser::multispace_or_comment1,
                ),
                Type::parse,
            ),
            preceded(parser::multispace_or_comment0, tag(")")),
        )(input)?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        // TODO; return error instead.
        assert!(types.len() > 1, "Found union with only a single type");

        Ok((
            input,
            UnionType {
                ext_attrs,
                types,
                nullable,
            },
        ))
    }
}

impl Parser<StandardType> for StandardType {
    fn parse(input: &str) -> IResult<&str, StandardType> {
        let (input, ext_attrs) = parser::parse_ext_attrs(input)?;
        let (input, name) =
            preceded(parser::multispace_or_comment0, StandardTypeName::parse)(input)?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        Ok((
            input,
            StandardType {
                ext_attrs,
                name,
                nullable,
            },
        ))
    }
}

impl Parser<StandardTypeName> for StandardTypeName {
    fn parse(input: &str) -> IResult<&str, StandardTypeName> {
        let (input, primitive_type_with_space) = preceded(
            parser::multispace_or_comment0,
            opt(alt((
                tag("unsigned short"),
                tag("unsigned long long"),
                tag("unsigned long"),
                tag("long long"),
                tag("unrestricted float"),
                tag("unrestricted double"),
            ))),
        )(input)?;

        if let Some(name) = primitive_type_with_space {
            match name {
                "unsigned short" => return Ok((input, StandardTypeName::UnsignedShort)),
                "unsigned long long" => return Ok((input, StandardTypeName::UnsignedLongLong)),
                "unsigned long" => return Ok((input, StandardTypeName::UnsignedLong)),
                "long long" => return Ok((input, StandardTypeName::LongLong)),
                "unrestricted float" => return Ok((input, StandardTypeName::UnrestrictedFloat)),
                "unrestricted double" => return Ok((input, StandardTypeName::UnrestrictedDouble)),
                _ => panic!(
                    "Found unexpected name for primitive type with space: {}",
                    name
                ),
            }
        }

        let (input, name) = parser::parse_identifier(input)?;
        match name {
            "any" => Ok((input, StandardTypeName::Any)),
            "undefined" => Ok((input, StandardTypeName::Undefined)),
            "boolean" => Ok((input, StandardTypeName::Boolean)),
            "byte" => Ok((input, StandardTypeName::Byte)),
            "octet" => Ok((input, StandardTypeName::Octet)),
            "short" => Ok((input, StandardTypeName::Short)),
            // "unsigned short"
            "long" => Ok((input, StandardTypeName::Long)),
            // "unsigned long"
            // "long long"
            // "unsigned long long"
            "float" => Ok((input, StandardTypeName::Float)),
            // "unrestricted float"
            "double" => Ok((input, StandardTypeName::Double)),
            // "unrestricted double"
            "bigint" => Ok((input, StandardTypeName::Bigint)),
            "DOMString" => Ok((input, StandardTypeName::DOMString)),
            "ByteString" => Ok((input, StandardTypeName::ByteString)),
            "USVString" => Ok((input, StandardTypeName::USVString)),
            "object" => Ok((input, StandardTypeName::Object)),
            "symbol" => Ok((input, StandardTypeName::Symbol)),
            "ArrayBuffer" => Ok((input, StandardTypeName::ArrayBuffer)),
            "Int8Array" => Ok((input, StandardTypeName::Int8Array)),
            "Int16Array" => Ok((input, StandardTypeName::Int16Array)),
            "Int32Array" => Ok((input, StandardTypeName::Int32Array)),
            "Uint8Array" => Ok((input, StandardTypeName::Uint8Array)),
            "Uint16Array" => Ok((input, StandardTypeName::Uint16Array)),
            "Uint32Array" => Ok((input, StandardTypeName::Uint32Array)),
            "Uint8ClampedArray" => Ok((input, StandardTypeName::Uint8ClampedArray)),
            "BigInt64Array" => Ok((input, StandardTypeName::BigInt64Array)),
            "BigUint64Array" => Ok((input, StandardTypeName::BigUint64Array)),
            "Float32Array" => Ok((input, StandardTypeName::Float32Array)),
            "Float64Array" => Ok((input, StandardTypeName::Float64Array)),
            _ => Ok((input, StandardTypeName::Identifier(name.to_string()))),
        }
    }
}
