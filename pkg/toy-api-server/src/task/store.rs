//! Trait for log store operations.

use crate::store::error::StoreError;
use crate::store::kv::{Delete, Find, Put};
use crate::store::StoreConnection;
use async_trait::async_trait;
use std::fmt;
use std::future::Future;
use toy_api::selection::selector::Selector;
use toy_api::task::{PendingTask, TaskLog, Tasks};
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
#[async_trait]
pub trait TaskStoreOps<C>:
    Send
    + Sync
    + Pending<Con = C>
    + WatchPending<Con = C>
    + Find<Con = C>
    + Put<Con = C>
    + Delete<Con = C>
where
    C: StoreConnection,
{
}

/// Trait Composit log store operations.
pub trait TaskLogStoreOps<C>: Send + Sync + FindLog<Con = C> + List<Con = C>
where
    C: StoreConnection,
{
}

/// Create Pending task entity.
#[async_trait]
pub trait Pending {
    type Con: StoreConnection;

    /// Create Pending task entity.
    async fn pending(&self, con: Self::Con, key: String, v: PendingTask) -> Result<(), StoreError>;
}

/// Watch Pending task entity.
#[async_trait]
pub trait WatchPending {
    type Con: StoreConnection;
    type Stream: toy_h::Stream<Item = Result<Vec<PendingTask>, StoreError>> + Send + 'static;

    /// Watch Pending task entity.
    async fn watch_pending(
        &self,
        con: Self::Con,
        prefix: String,
    ) -> Result<Self::Stream, StoreError>;
}

/// Find task log.
pub trait FindLog {
    type Con: StoreConnection;
    type T: Future<Output = Result<Option<TaskLog>, Self::Err>> + Send;
    type Err: fmt::Debug + Send;

    /// Find task log by specified task id.
    fn find(&self, con: Self::Con, task_id: TaskId, opt: FindOption) -> Self::T;
}

/// List task info.
pub trait List {
    type Con: StoreConnection;
    type T: Future<Output = Result<Tasks, Self::Err>> + Send;
    type Err: fmt::Debug + Send;

    /// List task info by time span.
    fn list(&self, con: Self::Con, opt: ListOption) -> Self::T;
}

#[derive(Clone, Debug)]
pub struct FindOption {}

#[derive(Clone, Debug)]
pub struct ListOption {
    selection: Selector,
}

impl FindOption {
    pub fn new() -> Self {
        Self {}
    }
}

impl ListOption {
    pub fn new() -> Self {
        Self {
            selection: Selector::default(),
        }
    }

    pub fn with_field_selection(self, selection: Selector) -> Self {
        Self { selection, ..self }
    }

    pub fn selection(&self) -> &Selector {
        &self.selection
    }
}
