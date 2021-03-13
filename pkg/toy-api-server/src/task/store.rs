//! Trait for log store operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use std::fmt;
use std::future::Future;
use toy_api::task::{PendingEntity, TaskLogEntity, TasksEntity};
use toy_core::task::TaskId;

/// This trait represents the concept of a Task Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait TaskStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: TaskStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// This trait represents the concept of a Log Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait TaskLogStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: TaskLogStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait Composit log store operations.
pub trait TaskStoreOps<C>: Send + Sync + Pending<Con = C> + WatchPending<Con = C>
where
    C: StoreConnection,
{
}

/// Trait Composit log store operations.
pub trait TaskLogStoreOps<C>: Send + Sync + Find<Con = C> + List<Con = C>
where
    C: StoreConnection,
{
}

/// Create Pending task entity.
pub trait Pending {
    type Con: StoreConnection;
    type T: Future<Output = Result<(), Self::Err>> + Send;
    type Err: fmt::Debug + Send;

    /// Create Pending task entity.
    fn pending(&self, con: Self::Con, key: String, v: PendingEntity) -> Self::T;
}

/// Watch Pending task entity.
pub trait WatchPending {
    type Con: StoreConnection;
    type Stream: toy_h::Stream<Item = Result<Vec<PendingEntity>, Self::Err>> + Send + 'static;
    type T: Future<Output = Result<Self::Stream, Self::Err>> + Send + 'static;
    type Err: fmt::Debug + Send;

    /// Watch Pending task entity.
    fn watch_pending(&self, con: Self::Con, prefix: String) -> Self::T;
}

/// Find task log.
pub trait Find {
    type Con: StoreConnection;
    type T: Future<Output = Result<Option<TaskLogEntity>, Self::Err>> + Send;
    type Err: fmt::Debug + Send;

    /// Find task log by specified task id.
    fn find(&self, con: Self::Con, task_id: TaskId, opt: FindOption) -> Self::T;
}

/// List task info.
pub trait List {
    type Con: StoreConnection;
    type T: Future<Output = Result<TasksEntity, Self::Err>> + Send;
    type Err: fmt::Debug + Send;

    /// List task info by time span.
    fn list(&self, con: Self::Con, opt: ListOption) -> Self::T;
}

#[derive(Clone, Debug)]
pub struct FindOption {}

#[derive(Clone, Debug)]
pub struct ListOption {}

#[derive(Clone, Debug)]
pub struct QueryOption {}

impl FindOption {
    pub fn new() -> Self {
        Self {}
    }
}

impl ListOption {
    pub fn new() -> Self {
        Self {}
    }
}

impl QueryOption {
    pub fn new() -> Self {
        Self {}
    }
}
