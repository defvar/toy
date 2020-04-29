//! Data structure deserialization api.

pub use self::deser_ops::{DeserializeMapOps, DeserializeSeqOps, DeserializeVariantOps};
pub use self::deserializer::{
    Deserializable, DeserializableCore, DeserializableOwned, Deserializer,
};
pub use self::error::Error;
pub use self::visitor::Visitor;

pub mod from_primitive;

mod deser_ops;
mod deserializer;
pub mod discard;
mod error;
mod impl_builtin;
mod impl_map;
mod impl_primitive;
mod impl_seq;
mod visitor;
