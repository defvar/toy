//! Data structure serialization and deserialization api.

mod from_primitive;
pub mod schema;

pub use from_primitive::FromPrimitive;

pub mod export {
    pub use std::default::Default;
    pub use std::marker::PhantomData;
    pub use std::option::Option::{self, None, Some};
    pub use std::result::Result::{self, Err, Ok};
}

#[cfg(feature = "toy-pack-derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate toy_pack_derive;

#[cfg(feature = "toy-pack-derive")]
#[doc(hidden)]
pub use toy_pack_derive::*;
