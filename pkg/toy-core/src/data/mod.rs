//! This is a data type that is commonly used.
//!
//! It is used for passing data between nodes and serializing graph information.
//!

pub use self::frame::{Frame, FrameType, Signal};
pub use self::value::Value;
pub use toy_map::Map;

pub mod error;
mod frame;
mod value;
mod value_impl_pack;
mod value_impl_unpack;

pub mod schema;

pub use value_impl_pack::pack;
pub use value_impl_unpack::unpack;
