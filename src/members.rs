use crate::{internal::String, Argument, ExtendedAttribute, Type};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub enum Member {
    Constant(Constant),
    Attribute(Attribute),
    Operation(Operation),
    Constructor(Constructor),
    Stringifier(Stringifier),
    Iterable(Iterable),
    Maplike(Maplike),
    Setlike(Setlike),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Constant {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: String,
    pub value: ConstValue,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub enum ConstValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),

    NegativeInfinity,
    Infinity,
    NaN,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Attribute {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub special: Option<AttrSpecial>,
    pub r#type: Type,
    pub identifier: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub enum AttrSpecial {
    Static,
    Stringifier,
    Inherit,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Operation {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub special: Option<OpSpecial>,
    pub r#type: Type,
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub enum OpSpecial {
    Getter,
    Setter,
    Deleter,
    Static,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Constructor {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub arguments: Vec<Argument>,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Stringifier {
    pub ext_attrs: Vec<ExtendedAttribute>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Iterable {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#async: bool,
    pub key: Option<Type>,
    pub value: Type,
    pub arguments: Option<Vec<Argument>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Maplike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub key: Type,
    pub value: Type,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Deserialize, Serialize))]
pub struct Setlike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub r#type: Type,
}

/* Functionality implementations */

impl Member {
    pub fn identifier(&self) -> Option<&String> {
        match self {
            Self::Constant(constant) => Some(&constant.identifier),
            Self::Attribute(attribute) => Some(&attribute.identifier),
            Self::Operation(operation) => Some(&operation.identifier),
            _ => None,
        }
    }

    pub fn ext_attrs(&self) -> &Vec<ExtendedAttribute> {
        match self {
            Self::Constant(constant) => &constant.ext_attrs,
            Self::Attribute(attribute) => &attribute.ext_attrs,
            Self::Operation(operation) => &operation.ext_attrs,
            Self::Constructor(constructor) => &constructor.ext_attrs,
            Self::Stringifier(stringifier) => &stringifier.ext_attrs,
            Self::Iterable(iterable) => &iterable.ext_attrs,
            Self::Maplike(maplike) => &maplike.ext_attrs,
            Self::Setlike(setlike) => &setlike.ext_attrs,
        }
    }

    pub fn ext_attrs_mut(&mut self) -> &mut Vec<ExtendedAttribute> {
        match self {
            Self::Constant(constant) => &mut constant.ext_attrs,
            Self::Attribute(attribute) => &mut attribute.ext_attrs,
            Self::Operation(operation) => &mut operation.ext_attrs,
            Self::Constructor(constructor) => &mut constructor.ext_attrs,
            Self::Stringifier(stringifier) => &mut stringifier.ext_attrs,
            Self::Iterable(iterable) => &mut iterable.ext_attrs,
            Self::Maplike(maplike) => &mut maplike.ext_attrs,
            Self::Setlike(setlike) => &mut setlike.ext_attrs,
        }
    }
}
