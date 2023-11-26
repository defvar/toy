#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod config;
mod plugin;
pub mod service;

pub use plugin::{all, stdin, stdout};
