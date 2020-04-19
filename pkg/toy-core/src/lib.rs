#[macro_use]
extern crate failure;

pub use self::service_type::ServiceType;
pub use self::service_uri::Uri;

pub mod channel;
pub mod data;
pub mod error;
pub mod executor;
pub mod graph;
#[macro_use]
mod macros;
pub mod registry;
pub mod service;
mod service_type;
mod service_uri;

pub mod prelude {
    pub use super::channel::{Incoming, Outgoing};
    pub use super::data::{self, Frame, Map, Value};
    pub use super::error::ServiceError;
    pub use super::executor::{AsyncRuntime, Executor};
    pub use super::graph::Graph;
    pub use super::registry::Registry;
    pub use super::service_type::ServiceType;
    pub use super::service_uri::Uri;
    pub use super::{factory, map_value, seq_value};
    #[doc(hidden)]
    pub use toy_pack::deser::from_primitive::FromPrimitive;
}
