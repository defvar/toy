use crate::authentication::{authenticate, Auth};
use crate::common::{self, body};
use crate::task::handlers;
use crate::task::store::{TaskLogStore, TaskStore};
use toy_api::task::{AllocateOption, ListOption, LogOption, PostOption, WatchOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for tasks api.
pub fn tasks<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    log_store: impl TaskLogStore<T>,
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    tasks_create(auth.clone(), client.clone(), task_store.clone())
        .or(tasks_list(auth.clone(), client.clone(), log_store.clone()))
        .or(tasks_log(auth.clone(), client.clone(), log_store))
        .or(tasks_watch(
            auth.clone(),
            client.clone(),
            task_store.clone(),
        ))
        .or(tasks_allocate(
            auth.clone(),
            client.clone(),
            task_store.clone(),
        ))
}

pub fn tasks_create<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks")
        .and(warp::post())
        .and(authenticate(auth, client))
        .and(common::query::query_opt::<PostOption>())
        .and(body::bytes())
        .and(with_task_store(task_store))
        .and_then(handlers::post)
}

pub fn tasks_watch<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks" / "watch")
        .and(warp::get())
        .and(authenticate(auth, client))
        .and(common::query::query_opt::<WatchOption>())
        .and(with_task_store(task_store))
        .and_then(handlers::watch)
}

pub fn tasks_allocate<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    task_store: impl TaskStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("tasks" / String / "allocate")
        .and(warp::post())
        .and(authenticate(auth, client))
        .and(common::query::query_opt::<AllocateOption>())
        .and(body::bytes())
        .and(with_task_store(task_store))
        .and_then(handlers::allocate)
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
        .and(common::query::query_opt::<ListOption>())
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
        .and(common::query::query_opt::<LogOption>())
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
