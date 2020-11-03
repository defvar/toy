use toy_core::prelude::Value;
use toy_core::registry::PortType;
use toy_pack::{Pack, Unpack};

#[derive(Debug, Pack, Unpack)]
pub struct GraphEntity {
    name: String,
    services: Vec<GraphNodeEntity>,
}

#[derive(Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct GraphNodeEntity {
    #[toy(rename = "type")]
    tp: String,
    uri: String,
    position: Position,
    port_type: Option<PortType>,
    config: Value,
    wires: Vec<String>,
}

#[derive(Debug, Default, Pack, Unpack)]
pub struct Position {
    x: u32,
    y: u32,
}
