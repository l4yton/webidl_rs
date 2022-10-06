use std::fmt;

use crate::{display, ternary, RecordType, StandardType, StandardTypeName, Type, UnionType};

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

        for (i, r#type) in self.types.iter().enumerate() {
            result.push_str(&r#type.to_string());
            if i + 1 < number {
                result.push_str(" or ");
            }
        }

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

impl fmt::Display for StandardTypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StandardTypeName::Identifier(identifier) => write!(f, "{}", identifier),

            StandardTypeName::Any => write!(f, "any"),
            StandardTypeName::Undefined => write!(f, "undefined"),
            StandardTypeName::Boolean => write!(f, "boolean"),
            StandardTypeName::Byte => write!(f, "byte"),
            StandardTypeName::Octet => write!(f, "octet"),
            StandardTypeName::Short => write!(f, "short"),
            StandardTypeName::UnsignedShort => write!(f, "unsigned short"),
            StandardTypeName::Long => write!(f, "long"),
            StandardTypeName::UnsignedLong => write!(f, "unsigned long"),
            StandardTypeName::LongLong => write!(f, "long long"),
            StandardTypeName::UnsignedLongLong => write!(f, "unsigned long long"),
            StandardTypeName::Float => write!(f, "float"),
            StandardTypeName::UnrestrictedFloat => write!(f, "unrestricted float"),
            StandardTypeName::Double => write!(f, "double"),
            StandardTypeName::UnrestrictedDouble => write!(f, "unrestricted double"),
            StandardTypeName::Bigint => write!(f, "bigint"),
            StandardTypeName::DOMString => write!(f, "DOMString"),
            StandardTypeName::ByteString => write!(f, "ByteString"),
            StandardTypeName::USVString => write!(f, "USVString"),
            StandardTypeName::Object => write!(f, "object"),
            StandardTypeName::Symbol => write!(f, "symbol"),
            StandardTypeName::ArrayBuffer => write!(f, "ArrayBuffer"),
            StandardTypeName::Int8Array => write!(f, "Int8Array"),
            StandardTypeName::Int16Array => write!(f, "Int16Array"),
            StandardTypeName::Int32Array => write!(f, "Int32Array"),
            StandardTypeName::Uint8Array => write!(f, "Uint8Array"),
            StandardTypeName::Uint16Array => write!(f, "Uint16Array"),
            StandardTypeName::Uint32Array => write!(f, "Uint32Array"),
            StandardTypeName::Uint8ClampedArray => write!(f, "Uint8ClampedArray"),
            StandardTypeName::BigInt64Array => write!(f, "BigInt64Array"),
            StandardTypeName::BigUint64Array => write!(f, "BigUint64Array"),
            StandardTypeName::Float32Array => write!(f, "Float32Array"),
            StandardTypeName::Float64Array => write!(f, "Float64Array"),
        }
    }
}
