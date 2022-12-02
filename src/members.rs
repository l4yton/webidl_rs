use crate::{Argument, ExtendedAttribute, Type};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Constant {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: String,
    pub value: ConstValue,
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),

    NegativeInfinity,
    Infinity,
    NaN,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub special: Option<AttrSpecial>,
    pub r#type: Type,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub enum AttrSpecial {
    Static,
    Stringifier,
    Inherit,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub special: Option<OpSpecial>,
    pub r#type: Type,
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub enum OpSpecial {
    Static,
    Getter,
    Setter,
    Deleter,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Stringifer {
    pub ext_attrs: Vec<ExtendedAttribute>,
}

#[derive(Debug, Clone)]
pub struct Iterable {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#async: bool,
    pub key_type: Option<Type>,
    pub value_type: Type,
    pub arguments: Option<Vec<Argument>>,
}

#[derive(Debug, Clone)]
pub struct Maplike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub key_type: Type,
    pub value_type: Type,
}

#[derive(Debug, Clone)]
pub struct Setlike {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub readonly: bool,
    pub r#type: Type,
}

/* Functionality implementations. */

impl Member {
    pub fn get_identifier(&self) -> Option<&str> {
        match self {
            Member::Constant(constant) => Some(&constant.identifier),
            Member::Attribute(attribute) => Some(&attribute.identifier),
            Member::Operation(operation) => Some(&operation.identifier),
            _ => None,
        }
    }

    pub fn get_ext_attrs(&self) -> &Vec<ExtendedAttribute> {
        match self {
            Member::Constant(constant) => &constant.ext_attrs,
            Member::Attribute(attribute) => &attribute.ext_attrs,
            Member::Operation(operation) => &operation.ext_attrs,
            Member::Constructor(constructor) => &constructor.ext_attrs,
            Member::Stringifer(stringifier) => &stringifier.ext_attrs,
            Member::Iterable(iterable) => &iterable.ext_attrs,
            Member::Maplike(maplike) => &maplike.ext_attrs,
            Member::Setlike(setlike) => &setlike.ext_attrs,
        }
    }

    pub fn get_ext_attrs_mut(&mut self) -> &mut Vec<ExtendedAttribute> {
        match self {
            Member::Constant(constant) => &mut constant.ext_attrs,
            Member::Attribute(attribute) => &mut attribute.ext_attrs,
            Member::Operation(operation) => &mut operation.ext_attrs,
            Member::Constructor(constructor) => &mut constructor.ext_attrs,
            Member::Stringifer(stringifier) => &mut stringifier.ext_attrs,
            Member::Iterable(iterable) => &mut iterable.ext_attrs,
            Member::Maplike(maplike) => &mut maplike.ext_attrs,
            Member::Setlike(setlike) => &mut setlike.ext_attrs,
        }
    }
}

/* Trait implementations. */

// ...
// ...
