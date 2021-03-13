use crate::common::body;
use crate::task::handlers;
use crate::task::store::{TaskLogStore, TaskStore};
use toy_api::graph::GraphEntity;
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for tasks api.
pub fn tasks<T>(
    log_store: impl TaskLogStore<T>,
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    tasks_create(task_store.clone())
        .or(tasks_list(log_store.clone()))
        .or(tasks_log(log_store))
        .or(tasks_watch(task_store.clone()))
}

pub fn tasks_create<T>(
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks")
        .and(warp::post())
        .and(body::json::<GraphEntity>())
        .and(with_task_store(task_store))
        .and_then(|a, b| handlers::post(a, b))
}

pub fn tasks_watch<T>(
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks" / "watch")
        .and(warp::get())
        .and(with_task_store(task_store))
        .and_then(handlers::watch)
}

pub fn tasks_list<T>(
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks")
        .and(warp::get())
        .and(with_log_store(log_store))
        .and_then(handlers::tasks)
}

pub fn tasks_log<T>(
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks" / String / "log")
        .and(warp::get())
        .and(with_log_store(log_store))
        .and_then(|a, b| handlers::log(a, b))
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
