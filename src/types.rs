use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Type {
    Sequence(Box<Type>),
    Record(Record),
    Promise(Box<Type>),
    Union(Vec<Type>),
    FrozenArray(Box<Type>),
    ObservableArray(Box<Type>),
    Standard(StandardType),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    // Can only be "DOMString", "USVString" or "ByteString".
    pub key: Box<Type>,
    pub value: Box<Type>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandardType {
    pub name: String,
    pub nullable: bool,
}
