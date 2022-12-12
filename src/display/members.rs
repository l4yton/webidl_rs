use std::fmt;

use itertools::join;

use crate::{
    AttrSpecial, Attribute, ConstValue, Constant, Constructor, Iterable, Maplike, Member,
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
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(
            f,
            "const {} {} = {};",
            self.r#type, self.identifier, self.value,
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
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if let Some(special) = &self.special {
            write!(f, "{} ", special)?;
        }

        if self.readonly {
            write!(f, "readonly ")?;
        }

        write!(f, "attribute {} {};", self.r#type, self.identifier)
    }
}

impl fmt::Display for AttrSpecial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttrSpecial::Inherit => write!(f, "inherit"),
            AttrSpecial::Static => write!(f, "static"),
            AttrSpecial::Stringifier => write!(f, "stringifier"),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if let Some(special) = &self.special {
            write!(f, "{} ", special)?;
        }

        write!(
            f,
            "{} {}({});",
            self.r#type,
            self.identifier,
            join(&self.arguments, ", ")
        )
    }
}

impl fmt::Display for OpSpecial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpSpecial::Static => write!(f, "static"),
            OpSpecial::Getter => write!(f, "getter"),
            OpSpecial::Setter => write!(f, "setter"),
            OpSpecial::Deleter => write!(f, "deleter"),
        }
    }
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "constructor({});", join(&self.arguments, ", "),)
    }
}

impl fmt::Display for Stringifer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        write!(f, "stringifier;")
    }
}

impl fmt::Display for Iterable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.r#async {
            write!(f, "async ")?;
        }

        write!(f, "iterable<")?;

        if let Some(key_type) = &self.key_type {
            write!(f, "{}, ", key_type)?;
        }

        write!(f, "{}>;", self.value_type)
    }
}

impl fmt::Display for Maplike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.readonly {
            write!(f, "readonly ")?;
        }

        write!(f, "maplike<{}, {}>;", self.key_type, self.value_type)
    }
}

impl fmt::Display for Setlike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.ext_attrs.is_empty() {
            write!(f, "[{}] ", join(&self.ext_attrs, ", "))?;
        }

        if self.readonly {
            write!(f, "readonly ")?;
        }

        write!(f, "setlike<{}>;", self.r#type,)
    }
}
