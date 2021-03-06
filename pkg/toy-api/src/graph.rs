use crate::common::Format;
use toy_core::prelude::Value;
use toy_core::registry::PortType;
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

#[derive(Clone, Debug, Pack, Unpack)]
pub struct GraphsEntity {
    graphs: Vec<GraphEntity>,
    count: u32,
}

impl GraphsEntity {
    pub fn new(graphs: Vec<GraphEntity>) -> Self {
        let count = graphs.len() as u32;
        Self { graphs, count }
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct FindOption {
    format: Option<Format>,
}

impl FindOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct PutOption {
    format: Option<Format>,
}

impl PutOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct DeleteOption {
    format: Option<Format>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}
