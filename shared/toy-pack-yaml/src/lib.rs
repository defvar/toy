//! Implementation for YAML

#[macro_use]
extern crate failure;

mod decoder;
pub mod deser;
mod deser_ops;
mod deserializer;
pub mod error;
pub mod ser;
mod ser_ops;
mod serializer;

pub use self::deser::unpack;
pub use self::ser::pack_to_string;
