#![feature(type_alias_impl_trait, error_generic_member_access)]

//! # Backend Store Implementation for Etcd.

pub mod client;
pub mod error;
pub mod kv;
mod store;
pub mod txn;
pub mod watch;

pub use client::Client;
pub use store::{EtcdStore, EtcdStoreConnection};
