#![feature(backtrace, min_type_alias_impl_trait)]

//! # Backend Store Implementation for Etcd.

pub mod client;
pub mod error;
pub mod kv;
mod store;
pub mod txn;
pub mod watch;

pub use client::Client;
pub use store::{EtcdStore, EtcdStoreConnection};
