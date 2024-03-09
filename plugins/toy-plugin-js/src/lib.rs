#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod codec;
pub mod config;
mod plugin;

pub use plugin::js;

pub mod service;
