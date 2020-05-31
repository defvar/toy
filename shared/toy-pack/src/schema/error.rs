use std::fmt::Display;

use failure::Fail;

/// This Trait using `SchemaVisitor`.
/// It is used when an error occurs in the implementation of SchemaVisitor.
///
pub trait Error: Sized + Fail {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}
