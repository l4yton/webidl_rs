use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{map, not, opt, peek},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{
    parser, FrozenArrayType, ObservableArrayType, Parser, PromiseType, RecordType, RecordTypeKey,
    SequenceType, StandardType, StandardTypeName, Type, UnionType,
};

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

impl Parser<Type> for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        alt((
            map(SequenceType::parse, Type::Sequence),
            map(RecordType::parse, Type::Record),
            map(PromiseType::parse, Type::Promise),
            map(FrozenArrayType::parse, Type::FrozenArray),
            map(ObservableArrayType::parse, Type::ObservableArray),
            map(UnionType::parse, Type::Union),
            map(StandardType::parse, Type::Standard),
        ))(input)
    }
}

impl Parser<SequenceType> for SequenceType {
    fn parse(input: &str) -> IResult<&str, SequenceType> {
        let (input, r#type) = parse_parameterized_type(input, "sequence")?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        Ok((
            input,
            SequenceType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl Parser<RecordType> for RecordType {
    fn parse(input: &str) -> IResult<&str, RecordType> {
        // TODO: Clean this up.
        let (input, (key, value)) = delimited(
            delimited(tag("record"), parser::multispace_or_comment0, tag("<")),
            separated_pair(
                delimited(
                    parser::multispace_or_comment0,
                    alt((
                        map(tag("DOMString"), |_| RecordTypeKey::DOMString),
                        map(tag("USVString"), |_| RecordTypeKey::USVString),
                        map(tag("ByteString"), |_| RecordTypeKey::ByteString),
                    )),
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
            RecordType {
                key,
                value: Box::new(value),
            },
        ))
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

impl Parser<PromiseType> for PromiseType {
    fn parse(input: &str) -> IResult<&str, PromiseType> {
        let (input, r#type) = parse_parameterized_type(input, "Promise")?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        Ok((
            input,
            PromiseType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl Parser<FrozenArrayType> for FrozenArrayType {
    fn parse(input: &str) -> IResult<&str, FrozenArrayType> {
        let (input, r#type) = parse_parameterized_type(input, "FrozenArray")?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        Ok((
            input,
            FrozenArrayType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl Parser<ObservableArrayType> for ObservableArrayType {
    fn parse(input: &str) -> IResult<&str, ObservableArrayType> {
        let (input, r#type) = parse_parameterized_type(input, "ObservableArray")?;
        let (input, nullable) = map(opt(tag("?")), |o| o.is_some())(input)?;

        Ok((
            input,
            ObservableArrayType {
                r#type: Box::new(r#type),
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
        let (input, primitive_type_with_space) = opt(delimited(
            parser::multispace_or_comment0,
            alt((
                tag("unsigned short"),
                tag("unsigned long long"),
                tag("unsigned long"),
                tag("long long"),
                tag("unrestricted float"),
                tag("unrestricted double"),
            )),
            // A bit hacky, but there shouldn't be any other character that may be part of the
            // identifier. Example:
            // `long longMember` - "long" is the actual type and "longMember" the identifier.
            peek(not(take_while_m_n(1, 1, |s: char| {
                s.is_ascii_alphanumeric() || s == '_' || s == '-'
            }))),
        ))(input)?;

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
            // NOTE: Interpreting "void" as "undefined", see: https://github.com/whatwg/webidl/issues/60
            "undefined" | "void" => Ok((input, StandardTypeName::Undefined)),
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
