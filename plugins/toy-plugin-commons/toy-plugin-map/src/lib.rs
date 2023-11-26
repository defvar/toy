//! Toy Plugin for Transform Object.

#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod config;
mod plugin;
pub mod service;
pub mod transform;
pub mod typed;

pub use plugin::{
    all, indexing, mapping, naming, put, reindexing, remove_by_index, remove_by_name, rename,
    single_value, to_map, to_seq,
};
