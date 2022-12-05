use crate::store::error::StoreError;
use crate::store::task_event::*;
use crate::store::StoreConnection;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use toy_api::task::{TaskEvent, TaskEventList, Tasks};
use toy_core::task::TaskId;
use toy_h::HttpClient;

#[derive(Clone, Debug)]
pub struct BTreeLogStore<T> {
    con: Option<BTreeLogStoreConnection>,
    _t: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct BTreeLogStoreConnection {
    map: Arc<Mutex<BTreeMap<String, Vec<TaskEvent>>>>,
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

#[async_trait::async_trait]
impl TaskEventStoreOps for BTreeLogStoreOps {
    type Con = BTreeLogStoreConnection;
    type Err = StoreError;

    async fn find(
        &self,
        con: Self::Con,
        task_id: TaskId,
        _opt: FindOption,
    ) -> Result<Option<TaskEventList>, Self::Err> {
        let lock = con.map.lock().unwrap();
        match lock.get(&task_id.to_string()) {
            Some(v) => Ok(Some(TaskEventList::new(v.clone()))),
            None => Ok(None),
        }
    }

    async fn list(&self, _con: Self::Con, _opt: ListOption) -> Result<Tasks, Self::Err> {
        Ok(Tasks::new(Vec::new()))
    }

    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        _opt: CreateOption,
    ) -> Result<(), Self::Err> {
        let mut lock = con.map.lock().unwrap();
        for e in events {
            lock.entry(e.task_id().to_string())
                .or_insert_with(|| Vec::new())
                .push(e);
        }
        Ok(())
    }
}

impl<T> TaskEventStore<T> for BTreeLogStore<T>
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
