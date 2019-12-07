//! Data structure serialization api.

pub use self::error::Error;
pub use self::ser_ops::{
    SerializeMapOps,
    SerializeSeqOps,
    SerializeStructOps,
};
pub use self::serializer::{
    Serializable,
    Serializer,
};

mod serializer;
mod error;
mod impl_builtin;
mod impl_primitive;
mod impl_seq;
mod impl_map;
mod ser_ops;
