//! Trait for graph store operations.

use crate::store::error::StoreError;
use crate::store::kv::*;
use crate::store::StoreConnection;
use async_trait::async_trait;

/// This trait represents the concept of a Supervisor Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait SupervisorStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: SupervisorStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait Composit supervisor store operations.
#[async_trait]
pub trait SupervisorStoreOps<C>:
    Clone + Send + Sync + Find<Con = C> + List<Con = C> + Put<Con = C> + Delete<Con = C>
where
    C: StoreConnection,
{
}
