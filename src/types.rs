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
}

#[derive(Debug, Clone)]
pub struct RecordType {
    // Can only be "DOMString", "USVString" or "ByteString".
    pub key: Box<Type>,
    pub value: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct StandardType {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub name: String,
    pub nullable: bool,
}
