#![feature(backtrace, type_alias_impl_trait)]

//! Core module of Toy.

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

    pub use super::data::{self, Frame, Value};
    pub use super::error::ServiceError;
    pub use super::graph::Graph;
    pub use super::mpsc::{Incoming, Outgoing};
    pub use super::registry::{app, layer, App, Layered, NoopEntry, Plugin, PortType, Registry};
    pub use super::service::{
        FlowPort, Service, ServiceContext, ServiceFactory, SinkPort, SourcePort,
    };
    pub use super::service_type::ServiceType;
    pub use super::service_uri::Uri;
    pub use super::task::{TaskContext, TaskId};
    pub use super::{factory, map_value, seq_value};
    #[doc(hidden)]
    pub use toy_map::Map;
    #[doc(hidden)]
    pub use toy_pack::FromPrimitive;
}
