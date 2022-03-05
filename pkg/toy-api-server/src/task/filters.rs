use crate::authentication::{authenticate, Auth};
use crate::task::handlers;
use crate::task::store::{TaskLogStore, TaskStore};
use crate::warp::filters::BoxedFilter;
use toy_api::task::{LogOption, PostOption, TaskListOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for tasks api.
pub fn tasks<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    log_store: impl TaskLogStore<T> + 'static,
    task_store: impl TaskStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    tasks_create(auth.clone(), client.clone(), task_store.clone())
        .or(tasks_list(auth.clone(), client.clone(), log_store.clone()))
        .or(tasks_log(auth.clone(), client.clone(), log_store))
        .boxed()
}

pub fn tasks_create<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    task_store: impl TaskStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    warp::path!("tasks")
        .and(warp::post())
        .and(authenticate(auth, client))
        .and(toy_api_http_common::query::query_opt::<PostOption>())
        .and(toy_api_http_common::body::bytes())
        .and(with_task_store(task_store))
        .and_then(handlers::post)
        .boxed()
}

pub fn tasks_list<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks")
        .and(warp::get())
        .and(authenticate(auth, client))
        .and(toy_api_http_common::query::query_opt::<TaskListOption>())
        .and(with_log_store(log_store))
        .and_then(handlers::tasks)
}

pub fn tasks_log<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks" / String / "log")
        .and(warp::get())
        .and(authenticate(auth, client))
        .and(toy_api_http_common::query::query_opt::<LogOption>())
        .and(with_log_store(log_store))
        .and_then(handlers::log)
}

fn with_log_store<T>(
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = (impl TaskLogStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || log_store.clone())
}

fn with_task_store<T>(
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = (impl TaskStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || task_store.clone())
}
