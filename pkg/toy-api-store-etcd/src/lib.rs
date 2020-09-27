#![feature(backtrace, type_alias_impl_trait)]

//! # Backend Store Implementation for Etcd.

pub mod client;
pub mod error;
pub mod kv;
mod store;
pub mod txn;

pub use client::Client;
pub use store::{EtcdStoreConnection, EtcdStoreOpsFactory};
