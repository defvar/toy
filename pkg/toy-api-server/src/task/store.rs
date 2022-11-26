//! Trait for log store operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use std::fmt;
use std::future::Future;
use toy_api::selection::selector::Selector;
use toy_api::task::{TaskLog, Tasks};
use toy_core::task::TaskId;

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
pub trait TaskLogStoreOps<C>: Send + Sync + FindLog<Con = C> + List<Con = C>
where
    C: StoreConnection,
{
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
