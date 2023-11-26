#![feature(type_alias_impl_trait, error_generic_member_access, impl_trait_in_assoc_type)]

//! # toy-api-server
//!
//! Execution toy-api.
//! - CRUD for resources.
//! - Execute task, and get Log.
//! - list executable service.
//! ...etc
//!

mod common;
pub mod context;
mod server;

pub mod config;
pub mod graph;
pub mod metrics;
pub mod services;
pub mod store;
pub mod supervisors;
pub mod task;

pub mod authentication;
pub mod authorization;
pub mod initializer;
pub mod rbac;

pub use common::error::ApiError;
pub use config::ServerConfig;
pub use server::Server;

pub mod api {
    //! The `toy-api-server` apis.

    pub use super::graph;
    pub use super::metrics;
    pub use super::rbac;
    pub use super::services;
    pub use super::supervisors;
    pub use super::task;
}

#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use toy_core::task::TaskId;
#[doc(hidden)]
pub use toy_h;
