use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortType {
    Source(u32),
    Flow(u32, u32),
    Sink(u32),
}

impl PortType {
    pub fn flow() -> PortType {
        PortType::Flow(1, 1)
    }

    pub fn fan_in_flow(v: u32) -> PortType {
        PortType::Flow(v, 1)
    }

    pub fn fan_out_flow(v: u32) -> PortType {
        PortType::Flow(1, v)
    }

    pub fn fan_flow(i: u32, o: u32) -> PortType {
        PortType::Flow(i, o)
    }

    pub fn source() -> PortType {
        PortType::Source(1)
    }

    pub fn fan_out_source(v: u32) -> PortType {
        PortType::Source(v)
    }

    pub fn sink() -> PortType {
        PortType::Sink(1)
    }

    pub fn fan_in_sink(v: u32) -> PortType {
        PortType::Sink(v)
    }
}

impl Default for PortType {
    fn default() -> Self {
        PortType::flow()
    }
}
