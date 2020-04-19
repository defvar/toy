use crate::graph::GraphRegistry;
use std::convert::Infallible;
use toy_core::graph::Graph;

pub async fn list(g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    let graphs = g.lock().await;
    let graphs: Vec<Graph> = graphs.clone().into_iter().collect();
    Ok(format!("{:?}", graphs))
}
