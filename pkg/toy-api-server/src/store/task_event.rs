//! Trait for log store operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use chrono::{DateTime, Utc};
use std::fmt;
use toy_api::selection::selector::Predicate;
use toy_api::task::{TaskEvent, TaskEventList, TaskList};

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

    /// List task events by specified task id.
    async fn list_event(
        &self,
        con: Self::Con,
        opt: ListEventOption,
    ) -> Result<TaskEventList, Self::Err>;

    /// List task info by time span.
    async fn list_task(&self, con: Self::Con, opt: ListTaskOption) -> Result<TaskList, Self::Err>;

    /// create task events.
    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        opt: CreateOption,
    ) -> Result<(), Self::Err>;
}

#[derive(Clone, Debug)]
pub struct ListEventOption {
    name: Option<Predicate>,
    start: Option<DateTime<Utc>>,
    stop: Option<DateTime<Utc>>,
    limit: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct ListTaskOption {
    name: Option<Predicate>,
    start: Option<DateTime<Utc>>,
    stop: Option<DateTime<Utc>>,
    limit: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct CreateOption {}

impl ListEventOption {
    pub fn with(
        name: Option<Predicate>,
        start: Option<DateTime<Utc>>,
        stop: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            name,
            start,
            stop,
            limit,
        }
    }

    pub fn name(&self) -> Option<&Predicate> {
        self.name.as_ref()
    }

    pub fn start(&self) -> Option<&DateTime<Utc>> {
        self.start.as_ref()
    }

    pub fn stop(&self) -> Option<&DateTime<Utc>> {
        self.stop.as_ref()
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit.clone()
    }
}

impl ListTaskOption {
    pub fn with(
        name: Option<Predicate>,
        start: Option<DateTime<Utc>>,
        stop: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            name,
            start,
            stop,
            limit,
        }
    }

    pub fn name(&self) -> Option<&Predicate> {
        self.name.as_ref()
    }

    pub fn start(&self) -> Option<&DateTime<Utc>> {
        self.start.as_ref()
    }

    pub fn stop(&self) -> Option<&DateTime<Utc>> {
        self.stop.as_ref()
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit.clone()
    }
}

impl CreateOption {
    pub fn new() -> Self {
        Self {}
    }
}
