use super::service::*;
use toy_core::prelude::PortType;
use toy_core::service::FnPortType;

const NAME_SPACE: &str = &"plugin.common.fanout";

#[derive(Debug, Clone, Copy)]
pub struct FanOutFlowPort;

impl FnPortType for FanOutFlowPort {
    fn port_type() -> PortType {
        PortType::fan_out_flow(20)
    }
}

pub fn broadcast() -> (&'static str, &'static str, Broadcast) {
    (NAME_SPACE, "broadcast", Broadcast)
}
