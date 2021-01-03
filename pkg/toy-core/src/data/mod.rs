//! This is a data type that is commonly used.
//!
//! It is used for passing data between nodes and serializing graph information.
//!

pub use self::frame::{Frame, FrameType, Signal};
pub use self::map::Map;
pub use self::value::Value;

pub mod error;
mod frame;
mod map;
mod value;
mod value_impl_pack;
mod value_impl_unpack;

pub mod schema;

pub use value_impl_pack::pack;
pub use value_impl_unpack::unpack;

mod map_impl_pack;
mod map_impl_schema;
mod map_impl_unpack;
