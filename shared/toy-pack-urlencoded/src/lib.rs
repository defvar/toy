//! # toy-pack Implementation for urlencoded

#![feature(backtrace)]

mod deser;
mod deser_ops;
mod deserializer;
mod error;

mod part;
mod ser;
mod ser_ops;
mod serializer;

pub use deser::unpack;
pub use error::QueryParseError;
pub use ser::pack_to_string;
