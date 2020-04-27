//! Data structure serialization api.

pub use self::error::Error;
pub use self::ser_ops::{
    SerializeMapOps, SerializeSeqOps, SerializeStructOps, SerializeTupleVariantOps,
};
pub use self::serializer::{Serializable, Serializer};

mod error;
mod impl_builtin;
mod impl_map;
mod impl_primitive;
mod impl_seq;
mod ser_ops;
mod serializer;
