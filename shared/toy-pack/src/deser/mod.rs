//! Data structure deserialization api.

pub use self::deser_ops::{
    DeserializeMapOps,
    DeserializeSeqOps,
    DeserializeVariantOps,
};
pub use self::deserializer::{
    Deserializable,
    DeserializableOwned,
    Deserializer,
};
pub use self::error::Error;
pub use self::visitor::Visitor;

pub mod from_primitive;

mod deserializer;
mod error;
mod impl_builtin;
mod impl_primitive;
mod impl_seq;
mod impl_map;
mod deser_ops;
mod visitor;
pub mod discard;
