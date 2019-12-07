//! Data structure serialization and deserialization api.

pub mod deser;
pub mod ser;


pub mod export {
    pub use std::default::Default;
    pub use std::marker::PhantomData;
    pub use std::option::Option::{self, None, Some};
    pub use std::result::Result::{self, Err, Ok};
}
