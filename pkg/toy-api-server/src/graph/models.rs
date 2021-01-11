use toy::core::prelude::Value;
use toy::core::registry::PortType;
use toy::core::task::TaskId;
use toy::supervisor::RunTaskResponse;
use toy_pack::{Pack, Unpack};

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

#[derive(Clone, Debug, Default, Pack, Unpack)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, Pack)]
pub struct RunTaskEntity {
    task_id: TaskId,
}

impl RunTaskEntity {
    pub fn from(r: RunTaskResponse) -> Self {
        Self { task_id: r.id() }
    }
}
