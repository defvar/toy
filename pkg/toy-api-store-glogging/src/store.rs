use crate::constants;
use crate::error::StoreGLoggingError;
use std::future::Future;
use std::marker::PhantomData;
use toy_api::selection::Operator;
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
struct Views {
    task_detail: String,
    task_list: String,
}

#[derive(Clone, Debug)]
pub struct GLoggingStore<T> {
    con: Option<GloggingStoreConnection<T>>,
    views: Views,
}

#[derive(Clone, Debug)]
pub struct GloggingStoreConnection<T> {
    client: toy_glogging::Client<T>,
}

#[derive(Clone, Debug)]
pub struct GLoggingStoreOps<T> {
    views: Views,
    _t: PhantomData<T>,
}

impl<T> GLoggingStore<T>
where
    T: HttpClient,
{
    pub fn new() -> Self {
        let task_detail = view_name(constants::ENV_KEY_TOY_API_STORE_GLOGGING_VIEW_TASK_DETAIL);
        let task_list = view_name(constants::ENV_KEY_TOY_API_STORE_GLOGGING_VIEW_TASK_LIST);
        Self {
            con: None,
            views: Views {
                task_detail,
                task_list,
            },
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
            views: self.views.clone(),
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
        let log_name = self.views.task_detail.to_owned();
        async move {
            let token = toy_glogging::auth::request_token(
                con.client.raw(),
                toy_glogging::auth::Scope::LoggingRead,
            )
            .await?;

            let req = ListRequest::from_resource_name(log_name)
                .with_filter(format!("labels.task={}", task_id));

            let r = con.client.list(&token, req).await?;

            if r.entries().len() == 0 {
                Ok(None)
            } else {
                r.entries()
                    .iter()
                    .try_fold(Vec::new(), |mut vec, x| match x.json_payload() {
                        Some(jv) => {
                            let r = jv.as_object().and_then(|x| {
                                let msg = x.get("message").map(|j| j.as_str()).unwrap_or(None);
                                let t = x.get("target").map(|j| j.as_str()).unwrap_or(None);
                                let g = x.get("graph").map(|j| j.as_str()).unwrap_or(None);
                                let uri = x.get("uri").map(|j| j.as_str()).unwrap_or(None);
                                let level = x.get("level").map(|j| j.as_str()).unwrap_or(None);
                                match (msg, t, g, uri, level) {
                                    (Some(msg), Some(t), Some(g), uri, Some(level)) => {
                                        Some(TaskLogInner::new(msg, t, g, uri, level))
                                    }
                                    _ => None,
                                }
                            });
                            if r.is_some() {
                                vec.push(r.unwrap());
                            }
                            Ok(vec)
                        }
                        None => Ok(vec),
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

    fn list(&self, con: Self::Con, opt: ListOption) -> Self::T {
        let span = tracing::span!(Level::DEBUG, "list");
        let log_name = self.views.task_list.to_owned();

        async move {
            let token = toy_glogging::auth::request_token(
                con.client.raw(),
                toy_glogging::auth::Scope::LoggingRead,
            )
            .await?;

            let req = ListRequest::from_resource_name(log_name);

            let mut preds = Vec::new();
            for p in opt.selection().preds() {
                let op = match p.op() {
                    Operator::Eq => "=",
                    Operator::NotEq => "!=",
                    Operator::GreaterThan => ">",
                    Operator::LessThan => "<",
                    Operator::Contains => ":",
                };
                preds.push(format!("{} {} \"{}\"", p.field(), op, p.value()));
            }
            let req = req.with_filter(preds.join(" AND "));
            let r = con.client.list(&token, req).await?;

            let tasks = r
                .entries()
                .iter()
                .filter_map(|x| {
                    match (
                        x.label("task"),
                        x.label("operation"),
                        x.label("graph"),
                        x.label("container_name"),
                        x.timestamp(),
                    ) {
                        (Some(id), Some(op), Some(g), Some(cn), Some(timestamp)) => {
                            let r = TasksInner::new(id, op, timestamp.clone(), g, cn);
                            r.ok()
                        }
                        _ => None,
                    }
                })
                .collect();
            Ok(Tasks::new(tasks))
        }
        .instrument(span)
    }
}

fn view_name(key: &str) -> String {
    std::env::var(key).expect(&format!("not found view name. please set env {}", key))
}
