use super::service::*;
use toy_core::prelude::*;
use toy_core::registry::{PluginRegistry, PortType};

const NAME_SPACE: &str = &"plugin.common.fanout";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "broadcast",
        PortType::fan_out_flow(20),
        factory!(broadcast, BroadcastConfig, new_broadcast_context),
    )
}
