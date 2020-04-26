use std::sync::Arc;
use tokio::sync::Mutex;
use toy_core::graph::Graph;

pub type GraphRegistry = Arc<Mutex<Vec<Graph>>>;

pub fn new_graph_registry(g: Vec<Graph>) -> GraphRegistry {
    Arc::new(Mutex::new(g))
}

mod filters;
mod handlers;

pub use filters::graphs;
