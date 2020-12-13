use super::config::*;
use super::service::*;
use toy_core::prelude::{factory, plugin, PluginRegistry, PortType};

const NAME_SPACE: &str = &"plugin.common.collect";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "first",
        PortType::sink(),
        factory!(first, FirstConfig, new_first_context),
    )
    .with("last", PortType::sink(), || Last)
    .with("count", PortType::sink(), || Count)
}
