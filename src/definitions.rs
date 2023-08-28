use swc_atoms::JsWord;

use crate::{Member, Type};

#[derive(Debug, Clone)]
pub enum Definition {
    Interface(Interface),
    InterfaceMixin(InterfaceMixin),
    Includes(Includes),
    CallbackInterface(CallbackInterface),
    Namespace(Namespace),
    Dictionary(Dictionary),
    Enumeration(Enumeration),
    CallbackFunction(CallbackFunction),
    Typedef(Typedef),
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: JsWord,
    pub inheritance: Option<JsWord>,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMixin {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: JsWord,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Includes {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub interface: JsWord,
    pub mixin: JsWord,
}

#[derive(Debug, Clone)]
pub struct CallbackInterface {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: JsWord,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: JsWord,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: JsWord,
    pub inheritance: Option<JsWord>,
    pub members: Vec<DictionaryMember>,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: JsWord,
    pub values: Vec<JsWord>,
}

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: JsWord,
    pub r#type: Type,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Typedef {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: JsWord,
}

#[derive(Debug, Clone)]
pub struct DictionaryMember {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub required: bool,
    pub r#type: Type,
    pub identifier: JsWord,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Clone)]
pub struct ExtendedAttribute {
    pub identifier: JsWord,
    pub value: Option<ExtAttrValue>,
}

#[derive(Debug, Clone)]
pub enum ExtAttrValue {
    ArgumentList(Vec<Argument>),
    NamedArgumentList(NamedArgumentList),
    Identifier(JsWord),
    IdentifierList(Vec<JsWord>),

    Wildcard,
}

#[derive(Debug, Clone)]
pub struct NamedArgumentList {
    pub identifier: JsWord,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub optional: bool,
    pub r#type: Type,
    pub variadic: bool,
    pub identifier: JsWord,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Clone)]
pub enum DefaultValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(JsWord),

    Null,
    Infinity,
    NegativeInfinity,
    NaN,
    Undefined,
    Sequence,
    Dictionary,
}

/* Functionality implementations */

impl Definition {
    pub fn identifier(&self) -> Option<&JsWord> {
        match self {
            Self::Interface(interface) => Some(&interface.identifier),
            Self::InterfaceMixin(interface_mixin) => Some(&interface_mixin.identifier),
            Self::Includes(_) => None,
            Self::CallbackInterface(cb_interface) => Some(&cb_interface.identifier),
            Self::Namespace(namespace) => Some(&namespace.identifier),
            Self::Dictionary(dictionary) => Some(&dictionary.identifier),
            Self::Enumeration(r#enum) => Some(&r#enum.identifier),
            Self::CallbackFunction(cb_function) => Some(&cb_function.identifier),
            Self::Typedef(typedef) => Some(&typedef.identifier),
        }
    }

    pub fn ext_attrs(&self) -> &Vec<ExtendedAttribute> {
        match self {
            Self::Interface(interface) => &interface.ext_attrs,
            Self::InterfaceMixin(interface_mixin) => &interface_mixin.ext_attrs,
            Self::Includes(includes) => &includes.ext_attrs,
            Self::CallbackInterface(cb_interface) => &cb_interface.ext_attrs,
            Self::Namespace(namespace) => &namespace.ext_attrs,
            Self::Dictionary(dictionary) => &dictionary.ext_attrs,
            Self::Enumeration(r#enum) => &r#enum.ext_attrs,
            Self::CallbackFunction(cb_function) => &cb_function.ext_attrs,
            Self::Typedef(typedef) => &typedef.ext_attrs,
        }
    }

    pub fn ext_attrs_mut(&mut self) -> &mut Vec<ExtendedAttribute> {
        match self {
            Self::Interface(interface) => &mut interface.ext_attrs,
            Self::InterfaceMixin(interface_mixin) => &mut interface_mixin.ext_attrs,
            Self::Includes(includes) => &mut includes.ext_attrs,
            Self::CallbackInterface(cb_interface) => &mut cb_interface.ext_attrs,
            Self::Namespace(namespace) => &mut namespace.ext_attrs,
            Self::Dictionary(dictionary) => &mut dictionary.ext_attrs,
            Self::Enumeration(r#enum) => &mut r#enum.ext_attrs,
            Self::CallbackFunction(cb_function) => &mut cb_function.ext_attrs,
            Self::Typedef(typedef) => &mut typedef.ext_attrs,
        }
    }

    pub fn is_partial(&self) -> bool {
        match self {
            Self::Interface(interface) => interface.partial,
            Self::InterfaceMixin(interface_mixin) => interface_mixin.partial,
            Self::Namespace(namespace) => namespace.partial,
            Self::Dictionary(dictionary) => dictionary.partial,
            _ => false,
        }
    }
}

/* Trait implementations */
