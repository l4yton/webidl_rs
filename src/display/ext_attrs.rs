use std::fmt;

use crate::{display, parser, ExtAttrValue, ExtendedAttribute};

fn display_ext_attr_identifier(identifier: &str) -> String {
    if identifier.is_empty() {
        return "\"\"".to_string();
    }

    if parser::parse_identifier(identifier).is_ok() {
        return identifier.to_string();
    }

    return format!("\"{}\"", identifier);
}

impl fmt::Display for ExtendedAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            return write!(f, "{}{}", self.identifier, value);
        }

        write!(f, "{}", self.identifier)
    }
}

impl fmt::Display for ExtAttrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtAttrValue::ArgumentList(arguments) => {
                write!(f, "({})", display::display_arguments(arguments))
            }
            ExtAttrValue::NamedArgumentList(named_args_list) => write!(
                f,
                "={}({})",
                named_args_list.identifier,
                display::display_arguments(&named_args_list.arguments)
            ),
            ExtAttrValue::Identifier(identifier) => {
                write!(f, "={}", display_ext_attr_identifier(identifier))
            }
            ExtAttrValue::IdentifierList(identifier_list) => {
                let mut result = String::new();
                let number = identifier_list.len();

                for (i, identifier) in identifier_list.iter().enumerate() {
                    result.push_str(&display_ext_attr_identifier(identifier));
                    if i + 1 < number {
                        result.push(',');
                        result.push(' ');
                    }
                }

                write!(f, "=({})", result)
            }
            ExtAttrValue::Wildcard => write!(f, "=*"),
        }
    }
}
