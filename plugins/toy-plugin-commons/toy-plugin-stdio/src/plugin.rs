use super::config::*;
use super::service::*;
use toy_core::prelude::*;
use toy_core::registry::PortType;

const NAME_SPACE: &str = &"plugin.common.stdio";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "stdin",
        PortType::source(),
        factory!(stdin, StdinConfig, new_stdin_context),
    )
    .with(
        "stdout",
        PortType::sink(),
        factory!(stdout, StdoutConfig, new_stdout_context),
    )
}
