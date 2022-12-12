use std::fmt;

use itertools::{join, Itertools};

use crate::{
    parser, Argument, CallbackFunction, CallbackInterface, DefaultValue, Definition, Dictionary,
    DictionaryMember, Enumeration, ExtAttrValue, ExtendedAttribute, Includes, Interface,
    InterfaceMixin, NamedArgumentList, Namespace, Typedef,
};

// TODO: Find a better solution to determine if an identifier has to be in quotes.
fn display_ext_attr_identifier(identifier: &str) -> String {
    if identifier.is_empty() {
        return "\"\"".to_string();
    }

    if parser::parse_identifier(identifier).is_ok() {
        return identifier.to_string();
    }

    format!("\"{}\"", identifier)
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(interface) => write!(f, "{}", interface),
            Definition::InterfaceMixin(interface_mixin) => write!(f, "{}", interface_mixin),
            Definition::Includes(includes) => write!(f, "{}", includes),
            Definition::CallbackInterface(cb_interface) => write!(f, "{}", cb_interface),
            Definition::Namespace(namespace) => write!(f, "{}", namespace),
            Definition::Dictionary(dictionary) => write!(f, "{}", dictionary),
            Definition::Enumeration(enumeration) => write!(f, "{}", enumeration),
            Definition::CallbackFunction(cb_function) => write!(f, "{}", cb_function),
            Definition::Typedef(typedef) => write!(f, "{}", typedef),
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.partial {
            write!(f, "partial ")?;
        }

        write!(f, "interface {} ", self.identifier)?;

        if let Some(inheritance) = &self.inheritance {
            write!(f, ": {} ", inheritance)?;
        }

        write!(f, "{{")?;
        if !self.members.is_empty() {
            write!(f, "\n\t{}", join(&self.members, "\n\t"))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for InterfaceMixin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.partial {
            write!(f, "partial ")?;
        }

        write!(f, "interface mixin {} {{", self.identifier)?;
        if !self.members.is_empty() {
            write!(f, "\n\t{}", join(&self.members, "\n\t"))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for Includes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "{} includes {};", self.interface, self.mixin)
    }
}

impl fmt::Display for CallbackInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "callback interface {} {{", self.identifier)?;
        if !self.members.is_empty() {
            write!(f, "\n\t{}", join(&self.members, "\n\t"))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.partial {
            write!(f, "partial ")?;
        }

        write!(f, "namespace {} {{", self.identifier)?;
        if !self.members.is_empty() {
            write!(f, "\n\t{}", join(&self.members, "\n\t"))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.partial {
            write!(f, "partial ")?;
        }

        write!(f, "dictionary {} ", self.identifier)?;

        if let Some(inheritance) = &self.inheritance {
            write!(f, ": {} ", inheritance)?;
        }

        write!(f, "{{")?;
        if !self.members.is_empty() {
            write!(f, "\n\t{}", join(&self.members, "\n\t"))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for Enumeration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "enum {} {{", self.identifier)?;
        if !self.values.is_empty() {
            write!(f, "\n\t\"{}\"", join(&self.values, "\",\n\t\""))?;
        }
        write!(f, "\n}};")
    }
}

impl fmt::Display for CallbackFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(
            f,
            "callback {} = {} ({});",
            self.identifier,
            self.r#type,
            join(&self.arguments, ", ")
        )
    }
}

impl fmt::Display for Typedef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "typedef {} {};", self.r#type, self.identifier)
    }
}

impl fmt::Display for DictionaryMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.required {
            write!(f, "required ")?;
        }

        write!(f, "{} {}", self.r#type, self.identifier)?;

        if let Some(value) = &self.default {
            write!(f, " = {}", value)?;
        }

        write!(f, ";")
    }
}

impl fmt::Display for ExtendedAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            // Don't put the equal sign between identifier and value here, because
            // `ExtAttrValue::ArgumentList` doesn't have one.
            return write!(f, "{}{}", self.identifier, value);
        }

        write!(f, "{}", self.identifier)
    }
}

impl fmt::Display for ExtAttrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtAttrValue::ArgumentList(arguments) => {
                write!(f, "({})", join(arguments, ", "))
            }
            ExtAttrValue::NamedArgumentList(named_args_list) => write!(f, "={}", named_args_list),
            ExtAttrValue::Identifier(identifier) => {
                write!(f, "={}", display_ext_attr_identifier(identifier))
            }
            ExtAttrValue::IdentifierList(identifier_list) => {
                write!(
                    f,
                    "=({})",
                    identifier_list
                        .iter()
                        .map(
                            |identifier| if identifier.chars().all(|s| s.is_ascii_alphanumeric()) {
                                identifier.to_string()
                            } else {
                                format!("{:?}", identifier)
                            }
                        )
                        .join(", ")
                )
            }
            ExtAttrValue::Wildcard => write!(f, "=*"),
        }
    }
}

impl fmt::Display for NamedArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.identifier, join(&self.arguments, ", "))
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.optional {
            write!(f, "optional ")?;
        }

        write!(f, "{}", self.r#type)?;

        if self.variadic {
            write!(f, "...")?;
        }

        write!(f, " {}", self.identifier)?;

        if let Some(value) = &self.default {
            write!(f, " = {}", value)?;
        }

        Ok(())
    }
}

impl fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefaultValue::Boolean(boolean) => {
                write!(f, "{}", if *boolean { "true" } else { "false" })
            }
            DefaultValue::Integer(integer) => write!(f, "{}", integer),
            DefaultValue::Decimal(decimal) => write!(f, "{}", decimal),
            DefaultValue::String(string) => write!(f, "{:?}", string),
            DefaultValue::Null => write!(f, "null"),
            DefaultValue::Infinity => write!(f, "Infinity"),
            DefaultValue::NegativeInfinity => write!(f, "-Infinity"),
            DefaultValue::NaN => write!(f, "NaN"),
            DefaultValue::Undefined => write!(f, "undefined"),
            DefaultValue::Sequence => write!(f, "[]"),
            DefaultValue::Dictionary => write!(f, "{{}}"),
        }
    }
}
