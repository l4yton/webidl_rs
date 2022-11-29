use std::fmt;

use crate::{
    display, AttrSpecial, Attribute, ConstValue, Constant, Constructor, Iterable, Maplike, Member,
    OpSpecial, Operation, Setlike, Stringifer,
};

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Member::Constant(constant) => write!(f, "{}", constant),
            Member::Attribute(attribute) => write!(f, "{}", attribute),
            Member::Operation(operation) => write!(f, "{}", operation),
            Member::Constructor(constructor) => write!(f, "{}", constructor),
            Member::Stringifer(stringifer) => write!(f, "{}", stringifer),
            Member::Iterable(iterable) => write!(f, "{}", iterable),
            Member::Maplike(maplike) => write!(f, "{}", maplike),
            Member::Setlike(setlike) => write!(f, "{}", setlike),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}const {} {} = {};",
            ext_attrs_str, self.r#type, self.identifier, self.value,
        )
    }
}

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstValue::Boolean(boolean) => {
                write!(f, "{}", if *boolean { "true" } else { "false" })
            }
            ConstValue::Integer(integer) => write!(f, "{}", integer),
            ConstValue::Decimal(decimal) => write!(f, "{}", decimal),
            ConstValue::NegativeInfinity => write!(f, "-Infinity"),
            ConstValue::Infinity => write!(f, "Infinity"),
            ConstValue::NaN => write!(f, "NaN"),
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }
        let special_str = if let Some(special) = &self.special {
            match special {
                AttrSpecial::Inherit => "inherit ",
                AttrSpecial::Static => "static ",
                AttrSpecial::Stringifier => "stringifier ",
            }
        } else {
            ""
        };

        write!(
            f,
            "{}{}{}attribute {} {};",
            ext_attrs_str,
            special_str,
            if self.readonly { "readonly " } else { "" },
            self.r#type,
            self.identifier
        )
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }
        let special_str = if let Some(special) = &self.special {
            match special {
                OpSpecial::Static => "static ",
                OpSpecial::Getter => "getter ",
                OpSpecial::Setter => "setter ",
                OpSpecial::Deleter => "deleter ",
            }
        } else {
            ""
        };

        write!(
            f,
            "{}{}{} {}({});",
            ext_attrs_str,
            special_str,
            self.r#type,
            self.identifier,
            display::display_arguments(&self.arguments)
        )
    }
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}constructor({});",
            ext_attrs_str,
            display::display_arguments(&self.arguments),
        )
    }
}

impl fmt::Display for Stringifer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(f, "{}stringifier;", ext_attrs_str)
    }
}

impl fmt::Display for Iterable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        if let Some(key_type) = &self.key_type {
            return write!(
                f,
                "{}{}iterable<{}, {}>;",
                ext_attrs_str,
                if self.r#async { "async " } else { "" },
                key_type,
                self.value_type
            );
        }

        write!(
            f,
            "{}{}iterable<{}>;",
            ext_attrs_str,
            if self.r#async { "async " } else { "" },
            self.value_type
        )
    }
}

impl fmt::Display for Maplike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}{}maplike<{}, {}>;",
            ext_attrs_str,
            if self.readonly { "readonly " } else { "" },
            self.key_type,
            self.value_type
        )
    }
}

impl fmt::Display for Setlike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}{}setlike<{}>;",
            ext_attrs_str,
            if self.readonly { "readonly " } else { "" },
            self.r#type,
        )
    }
}
