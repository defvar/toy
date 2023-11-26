#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

pub mod config;
mod plugin;

mod count;
mod first;
mod last;

pub use plugin::{all, count, first, last};

pub mod service {
    pub use super::count::{Count, CountContext};
    pub use super::first::{First, FirstContext};
    pub use super::last::{Last, LastContext};
}
