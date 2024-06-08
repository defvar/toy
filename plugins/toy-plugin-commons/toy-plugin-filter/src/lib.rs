#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

//! "Filter" plugin.
//! Filtering by applying conditions to input values.

mod plugin;
mod filter;
pub mod config;
pub mod predicate;

pub use plugin::{filter};

pub mod service {
    pub use super::filter::{Filter, FilterContext};
}
