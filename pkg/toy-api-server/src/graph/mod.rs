use std::sync::Arc;
use tokio::sync::Mutex;
use toy_core::graph::Graph;

pub type GraphRegistry = Arc<Mutex<Vec<Graph>>>;

mod filters;
mod handlers;

pub use filters::graphs;
