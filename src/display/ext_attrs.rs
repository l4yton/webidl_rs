use std::fmt;

use crate::{display, ternary, ExtAttrValue, ExtendedAttribute};

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
                if identifier.is_empty() {
                    return write!(f, "=\"\"");
                }

                // Wrap the idenitifer in quotes if doesn't match the identifier regex.
                // NOTE: The check isn't 100% accurate, but suffices for now.
                if !identifier
                    .chars()
                    .all(|s| s.is_ascii_alphanumeric() || s == '_' || s == '-')
                {
                    return write!(f, "=\"{}\"", identifier);
                }

                write!(f, "={}", identifier)
            }
            ExtAttrValue::IdentifierList(identifier_list) => {
                let mut result = String::new();
                let number = identifier_list.len();

                for (i, identifier) in identifier_list.iter().enumerate() {
                    result.push_str(ternary!(identifier.is_empty(), "\"\"", identifier));
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
