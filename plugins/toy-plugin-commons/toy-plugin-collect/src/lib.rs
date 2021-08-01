#![feature(type_alias_impl_trait)]

pub mod config;
mod fn_service;
mod plugin;

mod count;
mod last;

pub mod service {
    pub use super::count::{Count, CountContext};
    pub use super::fn_service::{first, new_first_context, FirstContext};
    pub use super::last::{Last, LastContext};
}

pub use plugin::load;
