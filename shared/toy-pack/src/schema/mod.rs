//! Scan Schema api.

mod error;
mod impl_builtin;
mod impl_primitive;
mod visitor;

pub use self::error::Error;
pub use self::visitor::{EnumVisitor, SchemaVisitor, StructVisitor, TupleVariantVisitor};

pub trait Schema {
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor;
}

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum WrapTypes {
    Option,
    Vec,
}
