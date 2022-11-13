use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{map, not, opt, peek},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    parser, FrozenArrayType, ObservableArrayType, Parser, PrimitiveType, PromiseType, RecordType,
    RecordTypeKey, SequenceType, StandardType, StandardTypeName, Type, UnionType,
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
        let (input, (key, value)) = delimited(
            delimited(tag("record"), parser::multispace_or_comment0, tag("<")),
            separated_pair(
                preceded(
                    parser::multispace_or_comment0,
                    alt((
                        map(tag("DOMString"), |_| RecordTypeKey::DOMString),
                        map(tag("USVString"), |_| RecordTypeKey::USVString),
                        map(tag("ByteString"), |_| RecordTypeKey::ByteString),
                    )),
                ),
                delimited(
                    parser::multispace_or_comment0,
                    tag(","),
                    parser::multispace_or_comment0,
                ),
                Type::parse,
            ),
            preceded(parser::multispace_or_comment0, tag(">")),
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
        alt((
            map(PrimitiveType::parse, StandardTypeName::PrimitiveType),
            map(parser::parse_identifier, StandardTypeName::Identifier),
        ))(input)
    }
}

impl Parser<PrimitiveType> for PrimitiveType {
    fn parse(input: &str) -> IResult<&str, PrimitiveType> {
        terminated(
            alt((
                map(tag("unsigned short"), |_| PrimitiveType::UnsignedShort),
                map(tag("unsigned long long"), |_| {
                    PrimitiveType::UnsignedLongLong
                }),
                map(tag("unsigned long"), |_| PrimitiveType::UnsignedLong),
                map(tag("long long"), |_| PrimitiveType::LongLong),
                map(tag("unrestricted float"), |_| {
                    PrimitiveType::UnrestrictedFloat
                }),
                map(tag("unrestricted double"), |_| {
                    PrimitiveType::UnrestrictedDouble
                }),
                map(tag("any"), |_| PrimitiveType::Any),
                map(tag("undefinded"), |_| PrimitiveType::Undefined),
                // NOTE: Interpreting "void" as "undefined", see: https://github.com/whatwg/webidl/issues/60
                map(tag("void"), |_| PrimitiveType::Undefined),
                map(tag("boolean"), |_| PrimitiveType::Boolean),
                map(tag("byte"), |_| PrimitiveType::Byte),
                map(tag("octet"), |_| PrimitiveType::Octet),
                map(tag("short"), |_| PrimitiveType::Short),
                map(tag("long"), |_| PrimitiveType::Long),
                map(tag("float"), |_| PrimitiveType::Float),
                map(tag("double"), |_| PrimitiveType::Double),
                map(tag("bigint"), |_| PrimitiveType::Bigint),
                map(tag("DOMString"), |_| PrimitiveType::DOMString),
                map(tag("ByteString"), |_| PrimitiveType::ByteString),
                map(tag("USVString"), |_| PrimitiveType::USVString),
                // There is a limit of 21 parsers by alt().
                alt((
                    map(tag("object"), |_| PrimitiveType::Object),
                    map(tag("symbol"), |_| PrimitiveType::Symbol),
                    map(tag("ArrayBuffer"), |_| PrimitiveType::ArrayBuffer),
                    map(tag("Int8Array"), |_| PrimitiveType::Int8Array),
                    map(tag("Int16Array"), |_| PrimitiveType::Int16Array),
                    map(tag("Int32Array"), |_| PrimitiveType::Int32Array),
                    map(tag("Uint8Array"), |_| PrimitiveType::Uint8Array),
                    map(tag("Uint16Array"), |_| PrimitiveType::Uint16Array),
                    map(tag("Uint32Array"), |_| PrimitiveType::Uint32Array),
                    map(tag("Uint8ClampedArray"), |_| {
                        PrimitiveType::Uint8ClampedArray
                    }),
                    map(tag("BigInt64Array"), |_| PrimitiveType::BigInt64Array),
                    map(tag("BigUint64Array"), |_| PrimitiveType::BigUint64Array),
                    map(tag("Float32Array"), |_| PrimitiveType::Float32Array),
                    map(tag("Float64Array"), |_| PrimitiveType::Float64Array),
                )),
            )),
            // A bit hacky, but there shouldn't be any other character that may be part of the
            // identifier. Example:
            // `long longMember` - "long" is the actual type and "longMember" the identifier.
            peek(not(take_while_m_n(1, 1, |s: char| {
                s.is_ascii_alphanumeric() || s == '_' || s == '-'
            }))),
        )(input)
    }
}
