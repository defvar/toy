#![feature(type_alias_impl_trait)]

mod plugin;
pub mod service;

pub mod config {
    pub use crate::service::TickConfig;
}

pub use plugin::tick;
