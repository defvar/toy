use crate::task::handlers;
use crate::task::store::TaskLogStore;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for tasks api.
pub fn tasks<T>(
    log_store: impl TaskLogStore<T>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    tasks_running(tx.clone())
        .or(tasks_list(log_store.clone()))
        .or(tasks_log(log_store))
}

pub fn tasks_running(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks" / "running")
        .and(warp::get())
        .and(with_tx(tx))
        .and_then(handlers::running_tasks)
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

fn with_tx(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = (Outgoing<Request, ServiceError>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || tx.clone())
}

fn with_log_store<T>(
    log_store: impl TaskLogStore<T>,
) -> impl Filter<Extract = (impl TaskLogStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || log_store.clone())
}
