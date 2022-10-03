mod members;
mod parser;
mod types;

use nom::IResult;
use serde::{Deserialize, Serialize};

pub use members::*;
pub use types::*;

pub trait Parser<T> {
    fn parse(input: &str) -> IResult<&str, T>;
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Interface {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub inheritance: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceMixin {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Includes {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub interface: String,
    pub mixin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackInterface {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Namespace {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub inheritance: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dictionary {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub members: Vec<DictionaryMember>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Enumeration {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackFunction {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub r#type: Type,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Typedef {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryMember {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub required: bool,
    pub r#type: Type,
    pub identifier: String,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedAttribute {
    pub identifier: String,
    pub value: Option<ExtAttrValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ExtAttrValue {
    ArgumentList(Vec<Argument>),
    NamedArgumentList(NamedArgumentList),
    Identifier(String),
    IdentifierList(Vec<String>),

    Wildcard,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamedArgumentList {
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub optional: bool,
    pub r#type: Type,
    pub variadic: bool,
    pub identifier: String,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DefaultValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),

    Null,
    Infinity,
    NegativeInfinity,
    NaN,
    Undefined,
    Sequence,
    Dictionary,
}
