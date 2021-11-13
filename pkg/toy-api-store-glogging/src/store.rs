use crate::constants;
use crate::error::StoreGLoggingError;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use toy_api::task::{TaskLog, TaskLogInner, Tasks, TasksInner};
use toy_api_server::store::error::StoreError;
use toy_api_server::store::StoreConnection;
use toy_api_server::task::store::{
    FindLog, FindOption, List, ListOption, TaskLogStore, TaskLogStoreOps,
};
use toy_api_server::TaskId;
use toy_glogging::models::ListRequest;
use toy_h::HttpClient;
use tracing::{Instrument, Level};

#[derive(Clone, Debug)]
pub struct GLoggingStore<T> {
    con: Option<GloggingStoreConnection<T>>,
    log_name: String,
}

#[derive(Clone, Debug)]
pub struct GloggingStoreConnection<T> {
    client: toy_glogging::Client<T>,
}

#[derive(Clone, Debug)]
pub struct GLoggingStoreOps<T> {
    log_name: String,
    _t: PhantomData<T>,
}

impl<T> GLoggingStore<T>
where
    T: HttpClient,
{
    pub fn new() -> Self {
        let log_name =
            std::env::var(constants::ENV_KEY_TOY_API_STORE_GLOGGING_LOG_NAME).expect(&format!(
                "not found log name. please set env {}",
                constants::ENV_KEY_TOY_API_STORE_GLOGGING_LOG_NAME
            ));
        Self {
            con: None,
            log_name,
        }
    }
}

impl<T> TaskLogStore<T> for GLoggingStore<T>
where
    T: HttpClient,
{
    type Con = GloggingStoreConnection<T>;
    type Ops = GLoggingStoreOps<T>;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        GLoggingStoreOps {
            log_name: self.log_name.to_owned(),
            _t: PhantomData,
        }
    }

    fn establish(&mut self, client: T) -> Result<(), StoreError> {
        let c = toy_glogging::Client::from(client);
        self.con = Some(GloggingStoreConnection { client: c });
        Ok(())
    }
}

impl<T> StoreConnection for GloggingStoreConnection<T> where T: HttpClient {}

impl<T> TaskLogStoreOps<GloggingStoreConnection<T>> for GLoggingStoreOps<T> where T: HttpClient {}

impl<T> FindLog for GLoggingStoreOps<T>
where
    T: HttpClient,
{
    type Con = GloggingStoreConnection<T>;
    type T = impl Future<Output = Result<Option<TaskLog>, Self::Err>> + Send;
    type Err = StoreGLoggingError;

    fn find(&self, con: Self::Con, task_id: TaskId, _opt: FindOption) -> Self::T {
        let span = tracing::span!(Level::DEBUG, "find", task_id = %task_id);
        let log_name = self.log_name.to_owned();
        async move {
            let token = toy_glogging::auth::request_token(
                con.client.raw(),
                toy_glogging::auth::Scope::LoggingRead,
            )
            .await?;

            let req = ListRequest::from_resource_name(log_name)
                .with_filter(format!("labels.task_id={}", task_id));

            let r = con.client.list(&token, req).await?;

            if r.entries().len() == 0 {
                Ok(None)
            } else {
                r.entries()
                    .iter()
                    .try_fold(Vec::new(), |mut vec, x| {
                        match x
                            .json_payload_raw()
                            .map(|x| toy_pack_json::unpack::<TaskLogInner>(x.as_bytes()))
                        {
                            Some(Ok(v)) => {
                                vec.push(v);
                                Ok(vec)
                            }
                            Some(Err(e)) => Err(e.into()),
                            None => Ok(vec),
                        }
                    })
                    .map(|v| Some(TaskLog::new(task_id, v)))
            }
        }
        .instrument(span)
    }
}

impl<T> List for GLoggingStoreOps<T>
where
    T: HttpClient,
{
    type Con = GloggingStoreConnection<T>;
    type T = impl Future<Output = Result<Tasks, Self::Err>> + Send;
    type Err = StoreGLoggingError;

    fn list(&self, con: Self::Con, _opt: ListOption) -> Self::T {
        let span = tracing::span!(Level::DEBUG, "list");
        let log_name = self.log_name.to_owned();

        async move {
            let token = toy_glogging::auth::request_token(
                con.client.raw(),
                toy_glogging::auth::Scope::LoggingRead,
            )
            .await?;

            let req = ListRequest::from_resource_name(log_name)
                .with_filter("operation.last = true OR operation.first = true");

            let r = con.client.list(&token, req).await?;

            let tasks = r.entries().iter().fold(HashMap::new(), |mut map, x| {
                match (x.label("task_id"), x.operation(), x.timestamp()) {
                    (Some(id), Some(op), Some(timestamp)) => {
                        let e = TasksInner::new(id);
                        if e.is_err() {
                            map
                        } else {
                            let old = map.remove(id).unwrap_or_else(|| e.unwrap());
                            let new_entry = if op.is_last() {
                                old.with_ended_at(timestamp)
                            } else {
                                old.with_started_at(timestamp)
                            };
                            map.insert(id, new_entry);
                            map
                        }
                    }
                    _ => map,
                }
            });
            Ok(Tasks::new(tasks.into_iter().map(|(_, v)| v).collect()))
        }
        .instrument(span)
    }
}
