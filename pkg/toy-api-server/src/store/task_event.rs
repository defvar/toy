//! Trait for log store operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use std::fmt;
use toy_api::selection::selector::Selector;
use toy_api::task::{TaskEvent, TaskEventList, Tasks};
use toy_core::task::TaskId;

/// This trait represents the concept of a task event Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait TaskEventStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: TaskEventStoreOps<Con = Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait task event store operations.
#[async_trait::async_trait]
pub trait TaskEventStoreOps: Send + Sync {
    type Con: StoreConnection;
    type Err: fmt::Debug + Send;

    /// Find task events by specified task id.
    async fn find(
        &self,
        con: Self::Con,
        task_id: TaskId,
        opt: FindOption,
    ) -> Result<Option<TaskEventList>, Self::Err>;

    /// List task info by time span.
    async fn list(&self, con: Self::Con, opt: ListOption) -> Result<Tasks, Self::Err>;

    /// create task events.
    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        opt: CreateOption,
    ) -> Result<(), Self::Err>;
}

#[derive(Clone, Debug)]
pub struct FindOption {}

#[derive(Clone, Debug)]
pub struct ListOption {
    selection: Selector,
}

#[derive(Clone, Debug)]
pub struct CreateOption {}

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

impl CreateOption {
    pub fn new() -> Self {
        Self {}
    }
}
