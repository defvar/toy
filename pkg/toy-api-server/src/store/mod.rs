//! Backend store traits for toy-api-server.

pub mod error;
pub mod kv;
pub mod memory;

/// Marker Trait for Connection.
/// Store may be database or filesystem or other.
pub trait StoreConnection: Clone + Send + Sync {}
