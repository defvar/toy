//! Noop implementation for log store.
//!

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use crate::task::models::{TaskLogEntity, TasksEntity};
use crate::task::store::{Find, FindOption, List, ListOption, TaskLogStore, TaskLogStoreOps};
use std::marker::PhantomData;
use toy::core::task::TaskId;
use toy_h::HttpClient;

#[derive(Clone, Debug)]
pub struct NoopLogStore<T> {
    con: Option<NoopLogConnection>,
    _t: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct NoopLogConnection;

#[derive(Clone, Debug)]
pub struct NoopLogStoreOps;

impl<T> NoopLogStore<T> {
    pub fn new() -> Self {
        Self {
            con: None,
            _t: PhantomData,
        }
    }
}

impl<T> TaskLogStore<T> for NoopLogStore<T>
where
    T: HttpClient,
{
    type Con = NoopLogConnection;
    type Ops = NoopLogStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.as_ref().map(|x| x.clone())
    }

    fn ops(&self) -> Self::Ops {
        NoopLogStoreOps
    }

    fn establish(&mut self, _client: T) -> Result<(), StoreError> {
        if self.con.is_none() {
            self.con = Some(NoopLogConnection);
        }
        Ok(())
    }
}

impl StoreConnection for NoopLogConnection {}
impl TaskLogStoreOps<NoopLogConnection> for NoopLogStoreOps {}

impl Find for NoopLogStoreOps {
    type Con = NoopLogConnection;
    type T = std::future::Ready<Result<Option<TaskLogEntity>, Self::Err>>;
    type Err = StoreError;

    fn find(&self, _con: Self::Con, _task_id: TaskId, _opt: FindOption) -> Self::T {
        std::future::ready(Ok(None))
    }
}

impl List for NoopLogStoreOps {
    type Con = NoopLogConnection;
    type T = std::future::Ready<Result<TasksEntity, Self::Err>>;
    type Err = StoreError;

    fn list(&self, _con: Self::Con, _opt: ListOption) -> Self::T {
        std::future::ready(Ok(TasksEntity::new(Vec::new())))
    }
}
