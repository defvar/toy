#![feature(backtrace, min_type_alias_impl_trait)]

//! # toy-api-server
//!
//! Execution toy-api.
//! - CRUD for graphs.
//! - Execute task, and get Log.
//! - list executable service.
//!

mod common;
mod reject_handler;
mod server;

pub mod config;
pub mod graph;
pub mod services;
pub mod store;
pub mod supervisors;
pub mod task;

pub mod auth;

pub use common::error::ApiError;
pub use config::ServerConfig;
pub use server::Server;

pub mod api {
    //! The `toy-api-server` apis.

    pub use super::graph::graphs;
    pub use super::services::services;
    pub use super::supervisors::supervisors;
    pub use super::task::tasks;
}

#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use toy_core::task::TaskId;
#[doc(hidden)]
pub use toy_h;
#[doc(hidden)]
pub use warp;
