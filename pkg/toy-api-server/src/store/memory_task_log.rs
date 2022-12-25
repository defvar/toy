use crate::store::error::StoreError;
use crate::store::task_event::*;
use crate::store::StoreConnection;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use toy_api::selection::candidate::CandidatePart;
use toy_api::task::{TaskEvent, TaskEventList, TaskList};
use toy_core::data::Value;
use toy_h::HttpClient;

#[derive(Clone, Debug)]
pub struct BTreeTaskLogStore<T> {
    con: Option<BTreeTaskLogStoreConnection>,
    _t: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct BTreeTaskLogStoreConnection {
    map: Arc<Mutex<Vec<TaskEvent>>>,
}

#[derive(Clone, Debug)]
pub struct BTreeTaskLogStoreOps;

impl<T> BTreeTaskLogStore<T> {
    pub fn new() -> BTreeTaskLogStore<T> {
        Self {
            con: None,
            _t: PhantomData,
        }
    }
}

impl StoreConnection for BTreeTaskLogStoreConnection {}

#[async_trait::async_trait]
impl TaskEventStoreOps for BTreeTaskLogStoreOps {
    type Con = BTreeTaskLogStoreConnection;
    type Err = StoreError;

    async fn list_event(
        &self,
        con: Self::Con,
        opt: ListEventOption,
    ) -> Result<TaskEventList, Self::Err> {
        let lock = con.map.lock().unwrap();
        let mut result = vec![];
        for item in lock.iter() {
            let name_match = if let Some(name) = opt.name() {
                let r = name.is_match(&CandidatePart::new("name", Value::from(item.name())));
                if r.is_err() {
                    return Err(StoreError::error("predicate error, name field"));
                }
                r.unwrap()
            } else {
                true
            };
            let start_match = if let Some(start) = opt.start() {
                start <= item.timestamp()
            } else {
                true
            };
            let stop_match = if let Some(stop) = opt.stop() {
                stop >= item.timestamp()
            } else {
                true
            };

            if name_match && start_match && stop_match {
                result.push(item.clone());
            }
        }
        result.sort_by(|a, b| a.timestamp().cmp(b.timestamp()));
        if opt.limit().is_some() {
            result = result.into_iter().take(opt.limit().unwrap()).collect();
        }
        Ok(TaskEventList::new(result))
    }

    async fn list_task(
        &self,
        _con: Self::Con,
        _opt: ListTaskOption,
    ) -> Result<TaskList, Self::Err> {
        Ok(TaskList::new(Vec::new()))
    }

    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        _opt: CreateOption,
    ) -> Result<(), Self::Err> {
        let mut lock = con.map.lock().unwrap();
        lock.extend_from_slice(&events);
        Ok(())
    }
}

impl<T> TaskEventStore<T> for BTreeTaskLogStore<T>
where
    T: HttpClient,
{
    type Con = BTreeTaskLogStoreConnection;
    type Ops = BTreeTaskLogStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        BTreeTaskLogStoreOps
    }

    fn establish(&mut self, _client: T) -> Result<(), StoreError> {
        self.con = Some(BTreeTaskLogStoreConnection {
            map: Arc::new(Mutex::new(Vec::new())),
        });
        Ok(())
    }
}
