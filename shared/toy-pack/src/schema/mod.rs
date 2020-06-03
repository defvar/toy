//! Scan Schema api.

mod error;
mod impl_builtin;
mod impl_primitive;
mod visitor;

pub use self::error::Error;
pub use self::visitor::{EnumVisitor, SchemaVisitor, StructVisitor, TupleVariantVisitor};

/// The traits that the scannable data structure implements.
/// Several primitive types "impl" are provided by default.
///
pub trait Schema {
    fn scan<V>(name: &str, visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor;
}

/// Primitive type represented by schema.
///
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveTypes {
    Boolean,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    USize,
    ISize,
    F32,
    F64,
    String,
    Str,
    Char,
}

/// Wrap type represented by schema.
///
#[derive(Debug, Clone, PartialEq)]
pub enum WrapTypes {
    Option,
    Vec,
}

/// Create a Schema using `SchemaVisitor`.
/// Please use the implementation according to the structure of the schema.
///
pub fn to_schema<T, V>(name: &str, visitor: V) -> Result<V::Value, V::Error>
where
    T: Schema,
    V: SchemaVisitor,
{
    T::scan(name, visitor)
}
