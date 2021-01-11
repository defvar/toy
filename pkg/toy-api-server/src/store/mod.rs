//! Backend store traits for toy-api-server.

use crate::store::error::StoreError;

pub mod error;

/// Marker Trait for Connection.
/// Store may be database or filesystem or other.
pub trait StoreConnection: Clone + Send + Sync {}

/// Trait for create `StoreOps` and connectiong store.
/// Depending on the specification of the store, it may be connecting to database or  prepare "InMemory Object".
pub trait StoreOpsFactory: Send + Sync {
    type Con: StoreConnection;
    type Ops: Clone + Send + Sync;

    /// Create StoreOps.
    fn create(&self) -> Result<Self::Ops, StoreError>;

    /// Connecting Store.
    fn connect(&self) -> Result<Self::Con, StoreError>;
}
