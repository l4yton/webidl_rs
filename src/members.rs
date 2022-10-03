use serde::{Deserialize, Serialize};

use crate::{Argument, ExtendedAttribute, Type};

#[derive(Debug, Deserialize, Serialize)]
pub enum Member {
    Constant(Constant),
    Attribute(Attribute),
    Operation(Operation),
    Constructor(Constructor),
    Stringifer(Stringifer),
    Iterable(Iterable),
    Maplike(Maplike),
    Setlike(Setlike),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Constant {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: String,
    pub value: ConstValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConstValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),

    #[serde(rename = "-Infinity")]
    NegativeInfinity,
    Infinity,
    NaN,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub special: Option<AttrSpecial>,
    pub r#type: Type,
    pub identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AttrSpecial {
    Static,
    Stringifier,
    Inherit,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Operation {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub special: Option<OpSpecial>,
    pub r#type: Type,
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OpSpecial {
    Static,
    Getter,
    Setter,
    Deleter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Constructor {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stringifer {
    pub ext_attrs: Vec<ExtendedAttribute>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Iterable {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#async: bool,
    pub key_type: Option<Type>,
    pub value_type: Type,
    pub arguments: Option<Vec<Argument>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Maplike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub key_type: Type,
    pub value_type: Type,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setlike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub r#type: Type,
}
