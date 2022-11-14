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
    pub r#type: Box<Type>,
    pub nullable: bool,
}

// AFAIU, only Union and StandardType can have extended attributes.
// https://webidl.spec.whatwg.org/#idl-annotated-types
#[derive(Debug, Clone)]
pub struct UnionType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub types: Vec<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct RecordType {
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
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct FrozenArrayType {
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct ObservableArrayType {
    pub r#type: Box<Type>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct StandardType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub name: StandardTypeName,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardTypeName {
    Primitive(PrimitiveType),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}
