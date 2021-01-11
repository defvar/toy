use crate::constants;
use crate::error::StoreGLoggingError;
use std::collections::HashMap;
use std::future::Future;
use toy_api_server::store::error::StoreError;
use toy_api_server::store::StoreConnection;
use toy_api_server::task::models::{TaskLogEntity, TaskLogInner, TasksEntity, TasksInner};
use toy_api_server::task::store::{
    Find, FindOption, List, ListOption, TaskLogStore, TaskLogStoreOps,
};
use toy_api_server::TaskId;
use toy_glogging::models::ListRequest;
use tracing::{Instrument, Level};

#[derive(Clone, Debug)]
pub struct GLoggingStore {
    con: Option<GloggingStoreConnection>,
    log_name: String,
}

#[derive(Clone, Debug)]
pub struct GloggingStoreConnection {
    client: toy_glogging::Client,
}

#[derive(Clone, Debug)]
pub struct GLoggingStoreOps {
    log_name: String,
}

impl GLoggingStore {
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

impl TaskLogStore for GLoggingStore {
    type Con = GloggingStoreConnection;
    type Ops = GLoggingStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        GLoggingStoreOps {
            log_name: self.log_name.to_owned(),
        }
    }

    fn establish(&mut self, client: reqwest::Client) -> Result<(), StoreError> {
        let c = toy_glogging::Client::from(client);
        self.con = Some(GloggingStoreConnection { client: c });
        Ok(())
    }
}

impl StoreConnection for GloggingStoreConnection {}

impl TaskLogStoreOps<GloggingStoreConnection> for GLoggingStoreOps {}

impl Find for GLoggingStoreOps {
    type Con = GloggingStoreConnection;
    type T = impl Future<Output = Result<Option<TaskLogEntity>, Self::Err>> + Send;
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

            let r = con.client.list(token, req).await?;

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
                    .map(|v| Some(TaskLogEntity::new(task_id, v)))
            }
        }
        .instrument(span)
    }
}

impl List for GLoggingStoreOps {
    type Con = GloggingStoreConnection;
    type T = impl Future<Output = Result<TasksEntity, Self::Err>> + Send;
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

            let r = con.client.list(token, req).await?;

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
            Ok(TasksEntity::new(
                tasks.into_iter().map(|(_, v)| v).collect(),
            ))
        }
        .instrument(span)
    }
}
