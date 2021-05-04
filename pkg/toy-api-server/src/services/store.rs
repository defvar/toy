use crate::store::error::StoreError;
use crate::store::kv::{Delete, Find, List, Put};
use crate::store::StoreConnection;
use async_trait::async_trait;

/// This trait represents the concept of a Service Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait ServiceStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: ServiceStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait Composit service store operations.
#[async_trait]
pub trait ServiceStoreOps<C>:
    Clone + Send + Sync + Find<Con = C> + List<Con = C> + Put<Con = C> + Delete<Con = C>
where
    C: StoreConnection,
{
}
