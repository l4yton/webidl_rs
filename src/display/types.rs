use std::fmt;

use crate::{
    display, ternary, FrozenArrayType, ObservableArrayType, PrimitiveType, PromiseType, RecordType,
    RecordTypeKey, SequenceType, StandardType, StandardTypeName, Type, UnionType,
};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Sequence(sequence) => write!(f, "{}", sequence),
            Type::Record(record) => write!(f, "{}", record),
            Type::Promise(promise) => write!(f, "{}", promise),
            Type::Union(r#union) => write!(f, "{}", r#union),
            Type::FrozenArray(frozen_array) => write!(f, "{}", frozen_array),
            Type::ObservableArray(observable_array) => {
                write!(f, "{}", observable_array)
            }
            Type::Standard(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl fmt::Display for SequenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sequence<{}>{}",
            self.r#type,
            ternary!(self.nullable, "?", "")
        )
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "record<{}, {}>", self.key, self.value)
    }
}

impl fmt::Display for RecordTypeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordTypeKey::DOMString => write!(f, "DOMString"),
            RecordTypeKey::USVString => write!(f, "USVString"),
            RecordTypeKey::ByteString => write!(f, "ByteString"),
        }
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

impl fmt::Display for PromiseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Promise<{}>{}",
            self.r#type,
            ternary!(self.nullable, "?", "")
        )
    }
}

impl fmt::Display for FrozenArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FrozenArray<{}>{}",
            self.r#type,
            ternary!(self.nullable, "?", "")
        )
    }
}

impl fmt::Display for ObservableArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObservableArray<{}>{}",
            self.r#type,
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
            StandardTypeName::PrimitiveType(primitive_type) => write!(f, "{}", primitive_type),
            StandardTypeName::Identifier(identifier) => write!(f, "{}", identifier),
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Any => write!(f, "any"),
            PrimitiveType::Undefined => write!(f, "undefined"),
            PrimitiveType::Boolean => write!(f, "boolean"),
            PrimitiveType::Byte => write!(f, "byte"),
            PrimitiveType::Octet => write!(f, "octet"),
            PrimitiveType::Short => write!(f, "short"),
            PrimitiveType::UnsignedShort => write!(f, "unsigned short"),
            PrimitiveType::Long => write!(f, "long"),
            PrimitiveType::UnsignedLong => write!(f, "unsigned long"),
            PrimitiveType::LongLong => write!(f, "long long"),
            PrimitiveType::UnsignedLongLong => write!(f, "unsigned long long"),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::UnrestrictedFloat => write!(f, "unrestricted float"),
            PrimitiveType::Double => write!(f, "double"),
            PrimitiveType::UnrestrictedDouble => write!(f, "unrestricted double"),
            PrimitiveType::Bigint => write!(f, "bigint"),
            PrimitiveType::DOMString => write!(f, "DOMString"),
            PrimitiveType::ByteString => write!(f, "ByteString"),
            PrimitiveType::USVString => write!(f, "USVString"),
            PrimitiveType::Object => write!(f, "object"),
            PrimitiveType::Symbol => write!(f, "symbol"),
            PrimitiveType::ArrayBuffer => write!(f, "ArrayBuffer"),
            PrimitiveType::Int8Array => write!(f, "Int8Array"),
            PrimitiveType::Int16Array => write!(f, "Int16Array"),
            PrimitiveType::Int32Array => write!(f, "Int32Array"),
            PrimitiveType::Uint8Array => write!(f, "Uint8Array"),
            PrimitiveType::Uint16Array => write!(f, "Uint16Array"),
            PrimitiveType::Uint32Array => write!(f, "Uint32Array"),
            PrimitiveType::Uint8ClampedArray => write!(f, "Uint8ClampedArray"),
            PrimitiveType::BigInt64Array => write!(f, "BigInt64Array"),
            PrimitiveType::BigUint64Array => write!(f, "BigUint64Array"),
            PrimitiveType::Float32Array => write!(f, "Float32Array"),
            PrimitiveType::Float64Array => write!(f, "Float64Array"),
        }
    }
}
