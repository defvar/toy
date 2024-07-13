//! Toy Plugin for System Stat.

#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

mod plugin;
mod service;
pub mod config;
mod collector;

pub use service::{Cpu, CpuContext, Memory, MemoryContext};
pub use plugin::{cpu, memory, all};
