//! Trait for graph store operations.

use crate::store::error::StoreError;
use crate::store::kv::*;
use crate::store::StoreConnection;

/// This trait represents the concept of a Graph Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait GraphStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: GraphStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait Composit graph store operations.
pub trait GraphStoreOps<C>:
    Clone + Send + Sync + Find<Con = C> + List<Con = C> + Put<Con = C> + Delete<Con = C>
where
    C: StoreConnection,
{
}
