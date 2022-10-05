use std::fmt;

use crate::{
    display, ternary, CallbackFunction, CallbackInterface, Definition, Dictionary,
    DictionaryMember, Enumeration, Includes, Interface, InterfaceMixin, Member, Namespace, Typedef,
};

fn display_inheritance(inheritance: &Option<String>) -> String {
    let mut result = String::new();
    if let Some(inheritance) = inheritance {
        result.push(':');
        result.push(' ');
        result.push_str(inheritance);
        result.push(' ');
    }

    result
}

fn display_members(members: &[Member]) -> String {
    let mut result = String::new();
    for member in members {
        result.push('\t');
        result.push_str(&member.to_string());
        result.push('\n');
    }

    result
}

fn display_dictionary_members(members: &[DictionaryMember]) -> String {
    let mut result = String::new();
    for member in members {
        result.push('\t');
        result.push_str(&member.to_string());
        result.push('\n');
    }

    result
}

fn display_enum_values(values: &Vec<String>) -> String {
    let mut result = String::new();
    let number = values.len();

    for (i, value) in values.iter().enumerate() {
        result.push('\t');
        result.push('"');
        result.push_str(&value.to_string());
        result.push('"');
        if i + 1 < number {
            result.push(',');
        }
        result.push('\n');
    }

    result
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
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}{}interface {} {}{{\n{}}};",
            ext_attrs_str,
            ternary!(self.partial, "partial ", ""),
            self.identifier,
            display_inheritance(&self.inheritance),
            display_members(&self.members),
        )
    }
}

impl fmt::Display for InterfaceMixin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}{}interface mixin {} {{\n{}}};",
            ext_attrs_str,
            ternary!(self.partial, "partial ", ""),
            self.identifier,
            display_members(&self.members),
        )
    }
}

impl fmt::Display for Includes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}{} includes {};",
            ext_attrs_str, self.interface, self.mixin
        )
    }
}

impl fmt::Display for CallbackInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}callback interface {} {{\n{}}};",
            ext_attrs_str,
            self.identifier,
            display_members(&self.members),
        )
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}{}namespace {} {}{{\n{}}};",
            ext_attrs_str,
            ternary!(self.partial, "partial ", ""),
            self.identifier,
            display_inheritance(&self.inheritance),
            display_members(&self.members),
        )
    }
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}{}dictionary {} {}{{\n{}}};",
            ext_attrs_str,
            ternary!(self.partial, "partial ", ""),
            self.identifier,
            display_inheritance(&self.inheritance),
            display_dictionary_members(&self.members),
        )
    }
}

impl fmt::Display for Enumeration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}enum {} {{\n{}}};",
            ext_attrs_str,
            self.identifier,
            display_enum_values(&self.values)
        )
    }
}

impl fmt::Display for CallbackFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}callback {} = {} ({});",
            ext_attrs_str,
            self.identifier,
            self.r#type,
            display::display_arguments(&self.arguments)
        )
    }
}

impl fmt::Display for Typedef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push('\n');
        }

        write!(
            f,
            "{}typedef {} {};",
            ext_attrs_str, self.r#type, self.identifier,
        )
    }
}
