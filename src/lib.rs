mod display;
mod members;
mod parser;
mod types;

use nom::{bytes::complete::tag, multi::separated_list0, sequence::delimited, IResult};

pub use members::*;
pub use types::*;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

pub(crate) use ternary;

pub trait Parser<T> {
    fn parse(input: &str) -> IResult<&str, T>;
}

pub fn parse(input: &str) -> IResult<&str, Vec<Definition>> {
    delimited(
        parser::multispace_or_comment0,
        separated_list0(
            delimited(
                parser::multispace_or_comment0,
                tag(";"),
                parser::multispace_or_comment0,
            ),
            Definition::parse,
        ),
        delimited(
            parser::multispace_or_comment0,
            tag(";"),
            parser::multispace_or_comment0,
        ),
    )(input)
}

pub fn to_string(definitions: &Vec<Definition>) -> String {
    definitions.iter().fold(String::new(), |mut a, b| {
        a.push_str(&b.to_string());
        a.push_str("\n\n");
        a
    })
}

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
    pub identifier: String,
    pub inheritance: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMixin {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Includes {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub interface: String,
    pub mixin: String,
}

#[derive(Debug, Clone)]
pub struct CallbackInterface {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub inheritance: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub partial: bool,
    pub identifier: String,
    pub inheritance: Option<String>,
    pub members: Vec<DictionaryMember>,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub identifier: String,
    pub r#type: Type,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Typedef {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub r#type: Type,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub struct DictionaryMember {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub required: bool,
    pub r#type: Type,
    pub identifier: String,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Clone)]
pub struct ExtendedAttribute {
    pub identifier: String,
    pub value: Option<ExtAttrValue>,
}

#[derive(Debug, Clone)]
pub enum ExtAttrValue {
    ArgumentList(Vec<Argument>),
    NamedArgumentList(NamedArgumentList),
    Identifier(String),
    IdentifierList(Vec<String>),

    Wildcard,
}

#[derive(Debug, Clone)]
pub struct NamedArgumentList {
    pub identifier: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub ext_attrs: Vec<ExtendedAttribute>,
    pub optional: bool,
    pub r#type: Type,
    pub variadic: bool,
    pub identifier: String,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Clone)]
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
