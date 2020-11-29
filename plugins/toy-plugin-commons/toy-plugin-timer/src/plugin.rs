use super::service::*;
use toy_core::prelude::*;
use toy_core::registry::PortType;

const NAME_SPACE: &str = &"plugin.common.timer";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "tick",
        PortType::source(),
        factory!(tick, TickConfig, new_tick_context),
    )
}
