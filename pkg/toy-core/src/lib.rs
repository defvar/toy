#![feature(backtrace, type_alias_impl_trait)]

pub use self::service_type::ServiceType;
pub use self::service_uri::Uri;

pub mod data;
pub mod error;
pub mod executor;
pub mod graph;
pub mod mpsc;
pub mod oneshot;
#[macro_use]
mod macros;
pub mod node_channel;
pub mod registry;
pub mod service;
mod service_type;
mod service_uri;
pub mod task;

pub mod prelude {
    //! The `toy-core` prelude.
    //!
    //! The purpose of this module is to alleviate imports of many common module.
    //!
    //! ```
    //! # #![allow(unused_imports)]
    //! use toy_core::prelude::*;
    //! ```

    pub use super::data::{self, Frame, Map, Value};
    pub use super::error::ServiceError;
    pub use super::graph::Graph;
    pub use super::mpsc::{Incoming, Outgoing};
    pub use super::registry::{
        app, plugin, App, Layered, Plugin, PluginRegistry, PortType, Registry,
    };
    pub use super::service::{Service, ServiceContext, ServiceFactory};
    pub use super::service_type::ServiceType;
    pub use super::service_uri::Uri;
    pub use super::task::{TaskContext, TaskId};
    pub use super::{factory, map_value, seq_value};
    #[doc(hidden)]
    pub use toy_pack::deser::from_primitive::FromPrimitive;
}
