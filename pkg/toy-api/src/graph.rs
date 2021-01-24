use toy::core::prelude::Value;
use toy::core::registry::PortType;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Default, Pack, Unpack)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct GraphEntity {
    name: String,
    services: Vec<GraphNodeEntity>,
}

#[derive(Clone, Debug, Pack, Unpack)]
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
