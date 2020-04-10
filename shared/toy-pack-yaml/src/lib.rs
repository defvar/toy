//! Implementation for YAML

#[macro_use]
extern crate failure;

mod decoder;
pub mod deser;
mod deser_ops;
mod deserializer;
pub mod error;

pub use self::deser::unpack;
