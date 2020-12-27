#![feature(backtrace, type_alias_impl_trait)]

//! # toy-api-server
//!
//! Execution toy-api.
//! - CRUD for graphs.
//! - execute graph.
//! - graph status.
//! - log.
//! - list executable service.
//!

mod graph;
mod service;
mod task;

pub use common::error::ApiError;
pub use server::Server;

pub mod api {
    //! The `toy-api-server` apis.

    pub use super::graph::graphs;
    pub use super::service::services;
    pub use super::task::tasks;
}

pub mod auth;
mod common;
mod server;
pub mod store;

#[doc(hidden)]
pub use reqwest;
