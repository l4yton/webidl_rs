use std::fmt;

use crate::{
    display, ternary, CallbackFunction, CallbackInterface, Definition, Dictionary,
    DictionaryMember, Enumeration, Includes, Interface, InterfaceMixin, Member, Namespace, Typedef,
};

fn display_inheritance(inheritance: &Option<String>) -> String {
    let mut result = String::new();
    if let Some(inheritance) = inheritance {
        result.push_str(":");
        result.push_str(inheritance);
        result.push_str(" ");
    }

    result
}

fn display_members(members: &Vec<Member>) -> String {
    members.iter().fold(String::new(), |mut a, b| {
        a.push_str("\t");
        a.push_str(&b.to_string());
        a.push_str("\n");
        a
    })
}

fn display_dictionary_members(members: &Vec<DictionaryMember>) -> String {
    members.iter().fold(String::new(), |mut a, b| {
        a.push_str("\t");
        a.push_str(&b.to_string());
        a.push_str("\n");
        a
    })
}

fn display_enum_values(values: &Vec<String>) -> String {
    values.iter().fold(String::new(), |mut a, b| {
        a.push_str("\t");
        a.push_str(&b.to_string());
        a.push_str("\n");
        a
    })
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
        }

        write!(
            f,
            "{}{}interface {} {{\n{}}};",
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
        }

        write!(
            f,
            "{}enum {} {{\n{}\n}};",
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
            ext_attrs_str.push_str(" ");
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
            ext_attrs_str.push_str(" ");
        }

        write!(
            f,
            "{}typedef {} {};",
            ext_attrs_str, self.r#type, self.identifier,
        )
    }
}
