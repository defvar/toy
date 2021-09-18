use serde::{Deserialize, Serialize};
use toy_core::prelude::Value;
use toy_core::registry::PortType;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    name: String,
    services: Vec<GraphNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphNode {
    #[serde(rename = "type")]
    tp: String,
    uri: String,
    position: Position,
    port_type: Option<PortType>,
    config: Value,
    wires: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphList {
    graphs: Vec<Graph>,
    count: u32,
}

impl GraphList {
    pub fn new(graphs: Vec<Graph>) -> Self {
        let count = graphs.len() as u32;
        Self { graphs, count }
    }
}
