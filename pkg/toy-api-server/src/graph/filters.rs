use crate::graph::handlers;
use crate::graph::GraphRegistry;
use warp::Filter;

pub fn graphs(
    g: GraphRegistry,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    graphs_list(g.clone())
    // .or(todos_create(db.clone()))
    // .or(todos_update(db.clone()))
    // .or(todos_delete(db))
}

pub fn graphs_list(
    g: GraphRegistry,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs")
        .and(warp::get())
        .and(with_graphs(g))
        .and_then(handlers::list)
}

fn with_graphs(
    g: GraphRegistry,
) -> impl Filter<Extract = (GraphRegistry,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || g.clone())
}
