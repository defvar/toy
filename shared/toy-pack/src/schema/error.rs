use std::fmt::Display;

/// This Trait using `SchemaVisitor`.
/// It is used when an error occurs in the implementation of SchemaVisitor.
///
pub trait Error: Sized {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}
