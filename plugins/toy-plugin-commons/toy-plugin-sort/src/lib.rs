#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod config;
pub mod service;
pub use plugin::{all, sort};

mod merge;
mod plugin;
