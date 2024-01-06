#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod config;
mod plugin;

mod fixed_size;

pub use plugin::fixed_size;

pub mod service {
    pub use super::fixed_size::{FixedSize, FixedSizeContext};
}
