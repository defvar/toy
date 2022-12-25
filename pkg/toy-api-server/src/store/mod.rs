//! Backend store traits for toy-api-server.

pub mod error;
pub mod kv;
pub mod memory;
pub mod memory_task_log;
pub mod task_event;

/// Marker Trait for Connection.
/// Store may be database or filesystem or other.
pub trait StoreConnection: Clone + Send + Sync {}
