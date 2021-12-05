//! A map of preserve order.
//!
//! By the map is backed by a [`IndexMap`].

mod map;
mod map_impl_pack;
mod map_impl_schema;
mod map_impl_unpack;

pub use self::map::Map;
