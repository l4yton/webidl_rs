use crate::ExtendedAttribute;

#[derive(Debug, Clone)]
pub enum Type {
    Sequence(Box<Type>),
    Record(RecordType),
    Promise(Box<Type>),
    Union(UnionType),
    FrozenArray(Box<Type>),
    ObservableArray(Box<Type>),
    Standard(StandardType),
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
pub struct StandardType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub name: StandardTypeName,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub enum StandardTypeName {
    Identifier(String),

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
