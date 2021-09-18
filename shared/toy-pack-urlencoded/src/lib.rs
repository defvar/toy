//! # toy-pack Implementation for urlencoded

#![feature(backtrace)]

mod deser;
mod error;

mod ser;

pub use deser::unpack;
pub use error::QueryParseError;
pub use ser::pack_to_string;
