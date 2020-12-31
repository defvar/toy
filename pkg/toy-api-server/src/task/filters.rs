use crate::store::{StoreConnection, StoreOpsFactory};
use crate::task::handlers;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use warp::Filter;

pub fn tasks<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    tasks_list(tx.clone()).or(tasks_run(con.clone(), ops.clone(), tx.clone()))
}

pub fn tasks_list(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::get())
        .and(with_tx(tx))
        .and_then(handlers::list)
}

pub fn tasks_run<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("tasks" / String / "run")
        .and(warp::post())
        .and(with_ops_and_tx(store, ops, tx))
        .and_then(|a, (b, c, d)| handlers::run(a, b, c, d))
}

fn with_tx(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = (Outgoing<Request, ServiceError>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || tx.clone())
}

fn with_ops_and_tx<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<
    Extract = ((
        C,
        impl StoreOpsFactory<C> + Clone,
        Outgoing<Request, ServiceError>,
    ),),
    Error = std::convert::Infallible,
> + Clone
where
    C: StoreConnection,
{
    warp::any().map(move || (con.clone(), ops.clone(), tx.clone()))
}
