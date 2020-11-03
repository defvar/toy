use super::config::*;
use super::service::*;
use toy_core::prelude::*;
use toy_core::registry::PortType;

const NAME_SPACE: &str = &"plugin.common.file";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "reader",
        PortType::source(),
        factory!(read, FileReadConfig, new_read_context),
    )
    .with(
        "writer",
        PortType::sink(),
        factory!(write, FileWriteConfig, new_write_context),
    )
}
