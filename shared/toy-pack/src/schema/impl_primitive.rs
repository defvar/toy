use super::{PrimitiveTypes, Schema, SchemaVisitor};

macro_rules! primitive_scan_impl {
    ($t: ident, $variant: expr) => {
        impl Schema for $t {
            fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
            where
                V: SchemaVisitor,
            {
                visitor.visit(name, $variant)
            }
        }
    };
}

primitive_scan_impl!(bool, PrimitiveTypes::Boolean);

primitive_scan_impl!(usize, PrimitiveTypes::USize);
primitive_scan_impl!(isize, PrimitiveTypes::ISize);

primitive_scan_impl!(u8, PrimitiveTypes::U8);
primitive_scan_impl!(u16, PrimitiveTypes::U16);
primitive_scan_impl!(u32, PrimitiveTypes::U32);
primitive_scan_impl!(u64, PrimitiveTypes::U64);
primitive_scan_impl!(i8, PrimitiveTypes::I8);
primitive_scan_impl!(i16, PrimitiveTypes::I16);
primitive_scan_impl!(i32, PrimitiveTypes::I32);
primitive_scan_impl!(i64, PrimitiveTypes::I64);

primitive_scan_impl!(String, PrimitiveTypes::String);

impl Schema for &str {
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        visitor.visit(name, PrimitiveTypes::Str)
    }
}
