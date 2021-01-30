//! Trait for graph store operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use std::fmt::Debug;
use std::future::Future;
use toy_api::graph::GraphEntity;

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

#[derive(Clone, Debug)]
pub struct FindOption {}

impl FindOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct ListOption {}

impl ListOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct PutOption {}

impl PutOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct DeleteOption {}

impl DeleteOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PutResult {
    Create,
    Update,
}

#[derive(Clone, Copy, Debug)]
pub enum DeleteResult {
    Deleted,
    NotFound,
}

/// Find one entity by specified key.
pub trait Find {
    type Con: StoreConnection;
    type T: Future<Output = Result<Option<GraphEntity>, Self::Err>> + Send;
    type Err: Debug + Send;

    /// Find one entity by specified key.
    fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T;
}

/// List all or part entities by specified prefix of key.
pub trait List {
    type Con: StoreConnection;
    type T: Future<Output = Result<Vec<GraphEntity>, Self::Err>> + Send;
    type Err: Debug + Send;

    /// List all or part entities by specified prefix of key.
    fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T;
}

/// Put one entity by specified key.
pub trait Put {
    type Con: StoreConnection;
    type T: Future<Output = Result<PutResult, Self::Err>> + Send;
    type Err: Debug + Send;

    /// Put one entity by specified key.
    fn put(&self, con: Self::Con, key: String, v: GraphEntity, opt: PutOption) -> Self::T;
}

/// Delete one entity by specified key.
pub trait Delete {
    type Con: StoreConnection;
    type T: Future<Output = Result<DeleteResult, Self::Err>> + Send;
    type Err: Debug + Send;

    /// Delete one entity by specified key.
    fn delete(&self, con: Self::Con, key: String, opt: DeleteOption) -> Self::T;
}
