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
    pub key: RecordKey,
    pub value: Box<Type>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RecordKey {
    DOMString,
    USVString,
    ByteString,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandardType {
    pub name: String,
    pub nullable: bool,
}
