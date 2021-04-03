use crate::store::error::StoreError;
use crate::store::StoreConnection;
use crate::task::store::*;
use std::collections::BTreeMap;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use toy_api::task::{TaskLogEntity, TasksEntity};
use toy_core::task::TaskId;
use toy_h::HttpClient;

#[derive(Clone, Debug)]
pub struct BTreeLogStore<T> {
    con: Option<BTreeLogStoreConnection>,
    _t: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct BTreeLogStoreConnection {
    map: Arc<Mutex<BTreeMap<String, TaskLogEntity>>>,
}

#[derive(Clone, Debug)]
pub struct BTreeLogStoreOps;

impl<T> BTreeLogStore<T> {
    pub fn new() -> BTreeLogStore<T> {
        Self {
            con: None,
            _t: PhantomData,
        }
    }
}

impl StoreConnection for BTreeLogStoreConnection {}

impl TaskLogStoreOps<BTreeLogStoreConnection> for BTreeLogStoreOps {}

impl<T> TaskLogStore<T> for BTreeLogStore<T>
where
    T: HttpClient,
{
    type Con = BTreeLogStoreConnection;
    type Ops = BTreeLogStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        BTreeLogStoreOps
    }

    fn establish(&mut self, _client: T) -> Result<(), StoreError> {
        self.con = Some(BTreeLogStoreConnection {
            map: Arc::new(Mutex::new(BTreeMap::new())),
        });
        Ok(())
    }
}

impl FindLog for BTreeLogStoreOps {
    type Con = BTreeLogStoreConnection;
    type T = impl Future<Output = Result<Option<TaskLogEntity>, Self::Err>> + Send;
    type Err = StoreError;

    fn find(&self, _con: Self::Con, _task_id: TaskId, _opt: FindOption) -> Self::T {
        async move { Ok(None) }
    }
}

impl List for BTreeLogStoreOps {
    type Con = BTreeLogStoreConnection;
    type T = impl Future<Output = Result<TasksEntity, Self::Err>> + Send;
    type Err = StoreError;

    fn list(&self, _con: Self::Con, _opt: ListOption) -> Self::T {
        async move { Ok(TasksEntity::new(Vec::new())) }
    }
}
