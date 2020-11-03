use super::config::*;
use super::service::*;
use toy_core::prelude::*;
use toy_core::registry::PortType;

const NAME_SPACE: &str = &"plugin.common.map";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "typed",
        PortType::flow(),
        factory!(typed, TypedConfig, new_typed_context),
    )
    .with(
        "mapping",
        PortType::flow(),
        factory!(mapping, MappingConfig, new_mapping_context),
    )
    .with(
        "naming",
        PortType::flow(),
        factory!(naming, NamingConfig, new_naming_context),
    )
    .with(
        "indexing",
        PortType::flow(),
        factory!(indexing, IndexingConfig, new_indexing_context),
    )
    .with(
        "reorder",
        PortType::flow(),
        factory!(reorder, ReorderConfig, new_reorder_context),
    )
}
