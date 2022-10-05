use std::fmt;

use crate::{display, ternary, RecordType, StandardType, Type, UnionType};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Sequence(sequence) => write!(f, "sequence<{}>", sequence),
            Type::Record(record) => write!(f, "{}", record),
            Type::Promise(promise) => write!(f, "Promise<{}>", promise),
            Type::Union(r#union) => write!(f, "{}", r#union),
            Type::FrozenArray(frozen_array) => write!(f, "FrozenArray<{}>", frozen_array),
            Type::ObservableArray(observable_array) => {
                write!(f, "ObservableArray<{}>", observable_array)
            }
            Type::Standard(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "record<{}, {}>", self.key, self.value)
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        let number = self.types.len();
        assert!(number > 1, "Found union with less than two types");
        self.types.iter().enumerate().for_each(|(i, r#type)| {
            result.push_str(&r#type.to_string());
            if i + 1 < number {
                result.push_str(" or ");
            }
        });

        write!(
            f,
            "{}({}){}",
            ext_attrs_str,
            result,
            ternary!(self.nullable, "?", "")
        )
    }
}

impl fmt::Display for StandardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ext_attrs_str = display::display_ext_attrs(&self.ext_attrs);
        if !ext_attrs_str.is_empty() {
            ext_attrs_str.push(' ');
        }

        write!(
            f,
            "{}{}{}",
            ext_attrs_str,
            self.name,
            ternary!(self.nullable, "?", "")
        )
    }
}
