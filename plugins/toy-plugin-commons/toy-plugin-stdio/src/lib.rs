#![feature(type_alias_impl_trait)]

pub mod config;
mod plugin;
pub mod service;

pub use plugin::{stdin, stdout};
