#![feature(backtrace, type_alias_impl_trait)]

//! # toy-api-server
//!
//! Execution toy-api.
//! - CRUD for graphs.
//! - Execute task, and get Log.
//! - list executable service.
//!

mod common;
mod server;

pub mod config;
pub mod graph;
pub mod service;
pub mod store;
pub mod task;

pub mod auth;

pub use common::error::ApiError;
pub use config::{DefaultConfig, ServerConfig};
pub use server::Server;

pub mod api {
    //! The `toy-api-server` apis.

    pub use super::graph::graphs;
    pub use super::service::services;
    pub use super::task::tasks;
}

#[doc(hidden)]
pub use reqwest;
#[doc(hidden)]
pub use toy::core::task::TaskId;
