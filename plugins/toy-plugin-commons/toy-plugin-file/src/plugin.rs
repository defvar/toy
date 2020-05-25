use super::config::*;
use super::service::*;
use toy_core::prelude::*;

const NAME_SPACE: &str = &"plugin.common.file";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "reader",
        factory!(read, FileReadConfig, new_read_context),
    )
    .service(
        "writer",
        factory!(write, FileWriteConfig, new_write_context),
    )
}
