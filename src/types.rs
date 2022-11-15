use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

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

/* Function implementations. */

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Sequence(sequence), Type::Sequence(other_sequence)) => {
                sequence == other_sequence
            }
            (Type::Record(record), Type::Record(other_record)) => record == other_record,
            (Type::Promise(promise), Type::Promise(other_promise)) => promise == other_promise,
            (Type::FrozenArray(frozen_array), Type::FrozenArray(other_frozen_array)) => {
                frozen_array == other_frozen_array
            }
            (
                Type::ObservableArray(observable_array),
                Type::ObservableArray(other_observable_array),
            ) => observable_array == other_observable_array,
            (Type::Standard(standard), Type::Standard(other_standard)) => {
                standard == other_standard
            }

            // Special handling for Union types.
            (Type::Union(r#union), _) | (_, Type::Union(r#union)) => r#union == other,

            _ => false,
        }
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Type::Sequence(sequence) => sequence.hash(state),
            Type::Record(record) => record.hash(state),
            Type::Promise(promise) => promise.hash(state),
            Type::Union(r#union) => r#union.hash(state),
            Type::FrozenArray(frozen_array) => frozen_array.hash(state),
            Type::ObservableArray(observable_array) => observable_array.hash(state),
            Type::Standard(standard) => standard.hash(state),
        }
    }
}

impl PartialEq for SequenceType {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl Eq for SequenceType {}

impl Hash for SequenceType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
    }
}

impl PartialEq for RecordType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for RecordType {}

impl Hash for RecordType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for PromiseType {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl Eq for PromiseType {}

impl Hash for PromiseType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
    }
}

impl PartialEq<Type> for UnionType {
    fn eq(&self, other: &Type) -> bool {
        self.types.iter().any(|r#type| r#type == other)
    }
}

// Unions are equal if:
// 1) The number of possible types match.
// 2) They contain the same types, regardless of order.
//
// This also makes sure that the following property holds true:
// k1 == k2 -> hash(k1) == hash(k2)
impl PartialEq for UnionType {
    fn eq(&self, other: &Self) -> bool {
        self.types.len() == other.types.len()
            && self
                .types
                .iter()
                .all(|r#type| other.types.iter().any(|other_type| r#type == other_type))
    }
}

// For the union type, we want that the hash for `(Foo or Bar)` == `(Bar or Foo)`,
// therefore we first compute the hash for each possible type, put them in a vector,
// sort it and use the resulting vector to calculate the final hash. ¯\_(ツ)_/¯
//
// NOTE: Can we optimize this?
impl Hash for UnionType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash_values = vec![];
        for r#type in &self.types {
            let mut hasher = DefaultHasher::new();
            r#type.hash(&mut hasher);
            hash_values.push(hasher.finish());
        }

        hash_values.sort_unstable();
        hash_values.hash(state);
    }
}

impl PartialEq for FrozenArrayType {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl Eq for FrozenArrayType {}

impl Hash for FrozenArrayType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
    }
}

impl PartialEq for ObservableArrayType {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl Eq for ObservableArrayType {}

impl Hash for ObservableArrayType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
    }
}

impl PartialEq for StandardType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for StandardType {}

impl Hash for StandardType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
