use std::fmt;

use crate::{ternary, Argument, DefaultValue, DictionaryMember, ExtendedAttribute};

pub(crate) fn display_ext_attrs(ext_attrs: &Vec<ExtendedAttribute>) -> String {
    let mut result = String::new();
    let number = ext_attrs.len();
    if number > 0 {
        result.push_str("[");
        result.push_str(&ext_attrs.iter().fold(String::new(), |mut a, b| {
            a.push_str(&b.to_string());
            a.push_str(", ");
            a
        }));
        result.push_str("]");
    }

    result
}

pub(crate) fn display_arguments(arguments: &Vec<Argument>) -> String {
    arguments.iter().fold(String::new(), |mut a, b| {
        a.push_str(&b.to_string());
        a.push_str(", ");
        a
    })
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push_str(" ");
        }
        let default = if let Some(value) = &self.default {
            value.to_string()
        } else {
            String::new()
        };

        write!(
            f,
            "{}{}{}{} {}{}",
            ext_attrs_str,
            ternary!(self.optional, "optional ", ""),
            &self.r#type,
            ternary!(self.variadic, "...", ""),
            self.identifier,
            default,
        )
    }
}

impl fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefaultValue::Boolean(boolean) => {
                write!(f, " = {}", ternary!(*boolean, "true", "false"))
            }
            DefaultValue::Integer(integer) => write!(f, " = {}", integer),
            DefaultValue::Decimal(decimal) => write!(f, " = {}", decimal),
            DefaultValue::String(string) => write!(f, " = \"{}\"", string,),
            DefaultValue::Null => write!(f, " = null"),
            DefaultValue::Infinity => write!(f, " = Infinity"),
            DefaultValue::NegativeInfinity => write!(f, " = -Infinity"),
            DefaultValue::NaN => write!(f, " = NaN"),
            DefaultValue::Undefined => write!(f, " = undefined"),
            DefaultValue::Sequence => write!(f, " = []"),
            DefaultValue::Dictionary => write!(f, " = {{}}"),
        }
    }
}

impl fmt::Display for DictionaryMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push_str(" ");
        }
        let default = if let Some(value) = &self.default {
            value.to_string()
        } else {
            String::new()
        };

        write!(
            f,
            "{}{}{} {}{};",
            ext_attrs_str,
            ternary!(self.required, "required ", ""),
            self.r#type,
            self.identifier,
            default,
        )
    }
}
