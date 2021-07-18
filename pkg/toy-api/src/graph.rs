use toy_core::prelude::Value;
use toy_core::registry::PortType;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Default, Pack, Unpack)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Graph {
    name: String,
    services: Vec<GraphNode>,
}

#[derive(Clone, Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct GraphNode {
    #[toy(rename = "type")]
    tp: String,
    uri: String,
    position: Position,
    port_type: Option<PortType>,
    config: Value,
    wires: Vec<String>,
}

#[derive(Clone, Debug, Pack, Unpack)]
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
