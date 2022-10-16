mod display;
mod members;
mod parser;
mod types;

use nom::{
    bytes::complete::tag,
    combinator::eof,
    error::Error,
    multi::separated_list0,
    sequence::{delimited, terminated},
    Err, IResult,
};

pub use members::*;
pub use types::*;

macro_rules! ternary {
    ($cond: expr, $a: expr, $b: expr) => {
        if $cond {
            $a
        } else {
            $b
        }
    };
}

pub(crate) use ternary;

pub trait Parser<T> {
    fn parse(input: &str) -> IResult<&str, T>;
}

pub fn parse_from_string(input: &str) -> Result<Vec<Definition>, Err<Error<String>>> {
    let (_, definitions) = terminated(
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
        ),
        eof,
    )(input)
    .map_err(|e| e.to_owned())?;

    Ok(definitions)
}

pub fn definitions_to_string(definitions: &[Definition]) -> String {
    let mut string = String::new();
    let number = definitions.len();

    for (i, definition) in definitions.iter().enumerate() {
        string.push_str(&definition.to_string());
        string.push('\n');
        if i + 1 < number {
            string.push('\n');
        }
    }

    string
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

impl Definition {
    pub fn get_identifier(&self) -> Option<&str> {
        match self {
            Definition::Interface(interface) => Some(&interface.identifier),
            Definition::InterfaceMixin(interface_mixin) => Some(&interface_mixin.identifier),
            Definition::Includes(_) => None,
            Definition::CallbackInterface(cb_interface) => Some(&cb_interface.identifier),
            Definition::Namespace(namespace) => Some(&namespace.identifier),
            Definition::Dictionary(dictionary) => Some(&dictionary.identifier),
            Definition::Enumeration(r#enum) => Some(&r#enum.identifier),
            Definition::CallbackFunction(cb_function) => Some(&cb_function.identifier),
            Definition::Typedef(typedef) => Some(&typedef.identifier),
        }
    }

    pub fn get_ext_attrs(&self) -> &Vec<ExtendedAttribute> {
        match self {
            Definition::Interface(interface) => &interface.ext_attrs,
            Definition::InterfaceMixin(interface_mixin) => &interface_mixin.ext_attrs,
            Definition::Includes(includes) => &includes.ext_attrs,
            Definition::CallbackInterface(cb_interface) => &cb_interface.ext_attrs,
            Definition::Namespace(namespace) => &namespace.ext_attrs,
            Definition::Dictionary(dictionary) => &dictionary.ext_attrs,
            Definition::Enumeration(r#enum) => &r#enum.ext_attrs,
            Definition::CallbackFunction(cb_function) => &cb_function.ext_attrs,
            Definition::Typedef(typedef) => &typedef.ext_attrs,
        }
    }

    pub fn get_ext_attrs_mut(&mut self) -> &mut Vec<ExtendedAttribute> {
        match self {
            Definition::Interface(interface) => &mut interface.ext_attrs,
            Definition::InterfaceMixin(interface_mixin) => &mut interface_mixin.ext_attrs,
            Definition::Includes(includes) => &mut includes.ext_attrs,
            Definition::CallbackInterface(cb_interface) => &mut cb_interface.ext_attrs,
            Definition::Namespace(namespace) => &mut namespace.ext_attrs,
            Definition::Dictionary(dictionary) => &mut dictionary.ext_attrs,
            Definition::Enumeration(r#enum) => &mut r#enum.ext_attrs,
            Definition::CallbackFunction(cb_function) => &mut cb_function.ext_attrs,
            Definition::Typedef(typedef) => &mut typedef.ext_attrs,
        }
    }

    pub fn is_partial(&self) -> bool {
        match self {
            Definition::Interface(interface) => interface.partial,
            Definition::InterfaceMixin(interface_mixin) => interface_mixin.partial,
            Definition::Namespace(namespace) => namespace.partial,
            Definition::Dictionary(dictionary) => dictionary.partial,
            _ => false,
        }
    }
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
