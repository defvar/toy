use toy_core::prelude::{app, PluginRegistry};

pub fn load() -> impl PluginRegistry {
    app(toy_plugin_fanout::load())
        .with(toy_plugin_file::load())
        .with(toy_plugin_map::load())
        .with(toy_plugin_timer::load())
        .with(toy_plugin_collect::load())
        .with(toy_plugin_stdio::load())
}
