#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

mod plugin;
pub mod service;

pub mod config {
    pub use crate::service::TickConfig;
}

pub use plugin::{all, tick};
