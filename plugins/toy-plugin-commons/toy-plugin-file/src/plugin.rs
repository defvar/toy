use super::config::*;
use super::service::*;
use toy_core::prelude::*;

pub fn load() -> impl PluginRegistry {
    plugin("reader", factory!(read, FileReadConfig, new_read_context)).service(
        "writer",
        factory!(write, FileWriteConfig, new_write_context),
    )
}
