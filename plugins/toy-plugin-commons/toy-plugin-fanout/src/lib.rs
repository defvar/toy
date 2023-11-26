#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

//! "FanOut" plugin.
//! Used to pass a value to multiple outputs or to split an input value.

mod plugin;
pub mod service;

pub use plugin::{all, broadcast, FanOutFlowPort};
