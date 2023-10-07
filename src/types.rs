use swc_atoms::JsWord;

use crate::ExtendedAttribute;

#[derive(Debug, Clone)]
pub enum Type {
    Sequence(SequenceType),
    Record(RecordType),
    Promise(PromiseType),
    Union(UnionType),
    FrozenArray(FrozenArrayType),
    ObservableArray(ObservableArrayType),
    Standard(StandardType),
}

#[derive(Debug, Clone)]
pub struct SequenceType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct UnionType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub types: Vec<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct RecordType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub key: RecordTypeKey,
    pub value: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum RecordTypeKey {
    DOMString,
    USVString,
    ByteString,
}

#[derive(Debug, Clone)]
pub struct PromiseType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct FrozenArrayType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct ObservableArrayType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct StandardType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub name: StandardTypeName,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub enum StandardTypeName {
    Primitive(PrimitiveType),
    Identifier(JsWord),
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Any,
    Undefined,
    Boolean,
    Byte,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    UnrestrictedFloat,
    Double,
    UnrestrictedDouble,
    Bigint,
    DOMString,
    ByteString,
    USVString,
    Object,
    Symbol,
    ArrayBuffer,
    Int8Array,
    Int16Array,
    Int32Array,
    Uint8Array,
    Uint16Array,
    Uint32Array,
    Uint8ClampedArray,
    BigInt64Array,
    BigUint64Array,
    Float32Array,
    Float64Array,
    DataView,
}

/* Functionality implementations */

impl Type {
    pub fn is_nullable(&self) -> bool {
        match self {
            Self::Sequence(sequence) => sequence.nullable,
            Self::Record(_record) => false,
            Self::Promise(promise) => promise.nullable,
            Self::Union(r#union) => r#union.nullable,
            Self::FrozenArray(frozen_array) => frozen_array.nullable,
            Self::ObservableArray(observable_array) => observable_array.nullable,
            Self::Standard(standard) => standard.nullable,
        }
    }
}

/* Trait implementations */

impl From<JsWord> for Type {
    fn from(identifier: JsWord) -> Self {
        Self::Standard(StandardType {
            ext_attrs: Vec::with_capacity(0),
            name: StandardTypeName::Identifier(identifier),
            nullable: false,
        })
    }
}

impl From<&JsWord> for Type {
    fn from(identifier: &JsWord) -> Self {
        Type::from(JsWord::from(identifier))
    }
}

impl From<String> for Type {
    fn from(identifier: String) -> Self {
        Type::from(JsWord::from(identifier))
    }
}

impl From<&str> for Type {
    fn from(identifier: &str) -> Self {
        Type::from(JsWord::from(identifier))
    }
}
