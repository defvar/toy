use crate::service::handlers;
use crate::service::ServiceRegistry;
use warp::Filter;

pub fn services<S, F>(
    reg: ServiceRegistry<S, F>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    S: Send + Sync,
    F: Send + Sync,
{
    service_list(reg.clone())
    // .or(todos_create(db.clone()))
    // .or(todos_update(db.clone()))
    // .or(todos_delete(db))
}

pub fn service_list<S, F>(
    reg: ServiceRegistry<S, F>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    S: Send + Sync,
    F: Send + Sync,
{
    warp::path!("services")
        .and(warp::get())
        .and(with_registry(reg))
        .and_then(handlers::list)
}

fn with_registry<S, F>(
    reg: ServiceRegistry<S, F>,
) -> impl Filter<Extract = (ServiceRegistry<S, F>,), Error = std::convert::Infallible> + Clone
where
    S: Send + Sync,
    F: Send + Sync,
{
    warp::any().map(move || reg.clone())
}
