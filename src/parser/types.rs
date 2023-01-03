use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, opt, peek, value, verify},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{
    parser, ExtendedAttribute, FrozenArrayType, ObservableArrayType, PrimitiveType, PromiseType,
    RecordType, RecordTypeKey, SequenceType, StandardType, StandardTypeName, Type, UnionType,
};

fn parse_parameterized_type<'a>(input: &'a str, name: &str) -> IResult<&'a str, Type> {
    delimited(
        tuple((
            parser::multispace_or_comment0,
            tag(name),
            parser::multispace_or_comment0,
            tag("<"),
        )),
        Type::parse,
        tuple((parser::multispace_or_comment0, tag(">"))),
    )(input)
}

impl Type {
    pub(crate) fn parse(input: &str) -> IResult<&str, Type> {
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

impl SequenceType {
    pub(crate) fn parse(input: &str) -> IResult<&str, SequenceType> {
        let (input, r#type) = parse_parameterized_type(input, "sequence")?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

        Ok((
            input,
            SequenceType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl RecordType {
    pub(crate) fn parse(input: &str) -> IResult<&str, RecordType> {
        let (input, (key, value)) = delimited(
            tuple((
                parser::multispace_or_comment0,
                tag("record"),
                parser::multispace_or_comment0,
                tag("<"),
            )),
            separated_pair(
                RecordTypeKey::parse,
                tuple((parser::multispace_or_comment0, tag(","))),
                Type::parse,
            ),
            tuple((parser::multispace_or_comment0, tag(">"))),
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

impl RecordTypeKey {
    pub(crate) fn parse(input: &str) -> IResult<&str, RecordTypeKey> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(tag("DOMString"), |_| RecordTypeKey::DOMString),
                map(tag("USVString"), |_| RecordTypeKey::USVString),
                map(tag("ByteString"), |_| RecordTypeKey::ByteString),
            )),
        )(input)
    }
}

impl UnionType {
    pub(crate) fn parse(input: &str) -> IResult<&str, UnionType> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, types) = delimited(
            tuple((
                parser::multispace_or_comment0,
                tag("("),
                parser::multispace_or_comment0,
            )),
            separated_list1(
                tuple((
                    parser::multispace_or_comment1,
                    tag("or"),
                    parser::multispace_or_comment1,
                )),
                Type::parse,
            ),
            tuple((parser::multispace_or_comment0, tag(")"))),
        )(input)?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

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

impl PromiseType {
    pub(crate) fn parse(input: &str) -> IResult<&str, PromiseType> {
        let (input, r#type) = parse_parameterized_type(input, "Promise")?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

        Ok((
            input,
            PromiseType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl FrozenArrayType {
    pub(crate) fn parse(input: &str) -> IResult<&str, FrozenArrayType> {
        let (input, r#type) = parse_parameterized_type(input, "FrozenArray")?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

        Ok((
            input,
            FrozenArrayType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl ObservableArrayType {
    pub(crate) fn parse(input: &str) -> IResult<&str, ObservableArrayType> {
        let (input, r#type) = parse_parameterized_type(input, "ObservableArray")?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

        Ok((
            input,
            ObservableArrayType {
                r#type: Box::new(r#type),
                nullable,
            },
        ))
    }
}

impl StandardType {
    pub(crate) fn parse(input: &str) -> IResult<&str, StandardType> {
        let (input, ext_attrs) = ExtendedAttribute::parse_multi0(input)?;
        let (input, name) = StandardTypeName::parse(input)?;
        let (input, nullable) = map(
            opt(tuple((parser::multispace_or_comment0, tag("?")))),
            |o| o.is_some(),
        )(input)?;

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

impl StandardTypeName {
    pub(crate) fn parse(input: &str) -> IResult<&str, StandardTypeName> {
        preceded(
            parser::multispace_or_comment0,
            alt((
                map(PrimitiveType::parse, StandardTypeName::Primitive),
                map(parser::parse_identifier, StandardTypeName::Identifier),
            )),
        )(input)
    }
}

impl PrimitiveType {
    pub(crate) fn parse(input: &str) -> IResult<&str, PrimitiveType> {
        terminated(
            alt((
                value(PrimitiveType::UnsignedShort, tag("unsigned short")),
                value(PrimitiveType::UnsignedLongLong, tag("unsigned long long")),
                value(PrimitiveType::UnsignedLong, tag("unsigned long")),
                value(PrimitiveType::LongLong, tag("long long")),
                value(PrimitiveType::UnrestrictedFloat, tag("unrestricted float")),
                value(
                    PrimitiveType::UnrestrictedDouble,
                    tag("unrestricted double"),
                ),
                value(PrimitiveType::Any, tag("any")),
                value(PrimitiveType::Undefined, tag("undefined")),
                // NOTE: Interpreting "void" as "undefined", see: https://github.com/whatwg/webidl/issues/60
                value(PrimitiveType::Undefined, tag("void")),
                value(PrimitiveType::Boolean, tag("boolean")),
                value(PrimitiveType::Byte, tag("byte")),
                value(PrimitiveType::Octet, tag("octet")),
                value(PrimitiveType::Short, tag("short")),
                value(PrimitiveType::Long, tag("long")),
                value(PrimitiveType::Float, tag("float")),
                value(PrimitiveType::Double, tag("double")),
                value(PrimitiveType::Bigint, tag("bigint")),
                value(PrimitiveType::DOMString, tag("DOMString")),
                value(PrimitiveType::ByteString, tag("ByteString")),
                value(PrimitiveType::USVString, tag("USVString")),
                // There is a limit of 21 parsers by alt().
                alt((
                    value(PrimitiveType::Object, tag("object")),
                    value(PrimitiveType::Symbol, tag("symbol")),
                    value(PrimitiveType::ArrayBuffer, tag("ArrayBuffer")),
                    value(PrimitiveType::Int8Array, tag("Int8Array")),
                    value(PrimitiveType::Int16Array, tag("Int16Array")),
                    value(PrimitiveType::Int32Array, tag("Int32Array")),
                    value(PrimitiveType::Uint8Array, tag("Uint8Array")),
                    value(PrimitiveType::Uint16Array, tag("Uint16Array")),
                    value(PrimitiveType::Uint32Array, tag("Uint32Array")),
                    value(PrimitiveType::Uint8ClampedArray, tag("Uint8ClampedArray")),
                    value(PrimitiveType::BigInt64Array, tag("BigInt64Array")),
                    value(PrimitiveType::BigUint64Array, tag("BigUint64Array")),
                    value(PrimitiveType::Float32Array, tag("Float32Array")),
                    value(PrimitiveType::Float64Array, tag("Float64Array")),
                )),
            )),
            // Make sure there isn't any character following that may be part of the identifier or
            // type.
            // Examples:
            // * `long longMember` - "long" is the actual type and "longMember" the identifier.
            // * `DOMStringList foo` - The type is "DOMStringList" and not "DOMString".
            peek(verify(take(1usize), |s: &str| {
                !(s.chars().all(|c| c.is_ascii_alphanumeric()) || s == "_" || s == "-")
            })),
        )(input)
    }
}
