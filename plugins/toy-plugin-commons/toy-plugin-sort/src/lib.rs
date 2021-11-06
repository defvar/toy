#![feature(type_alias_impl_trait)]

pub mod config;
pub mod service;
pub use plugin::{all, sort};

mod merge;
mod plugin;
