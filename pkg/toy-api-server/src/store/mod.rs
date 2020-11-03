//! Backend store api for toy-api-server.

pub use store_op::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult,
};

use crate::store::error::StoreError;

pub mod btree_store;
pub mod error;
mod store_op;

/// Connection for store.
/// Store may be database or filesystem or other.
pub trait StoreConnection: Clone + Send + Sync {}

/// Trait for all operations.
/// The traits that the store implements.
pub trait StoreOps<C>:
    Send + Sync + Find<Con = C> + List<Con = C> + Put<Con = C> + Delete<Con = C>
where
    C: StoreConnection,
{
}

/// Trait for create `StoreOps` and connectiong store.
/// Depending on the specification of the store, it may be connecting to database or  prepare "InMemory Object".
pub trait StoreOpsFactory<C>: Send + Sync
where
    C: StoreConnection,
{
    type Ops: StoreOps<C>;

    /// Create StoreOps.
    fn create(&self) -> Result<Self::Ops, StoreError>;

    /// Connecting Store.
    fn connect(&self) -> Result<C, StoreError>;
}
