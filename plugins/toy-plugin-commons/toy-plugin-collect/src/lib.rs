#![feature(type_alias_impl_trait)]

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
