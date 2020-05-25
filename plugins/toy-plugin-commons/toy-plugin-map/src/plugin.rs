use super::config::*;
use super::service::*;
use toy_core::prelude::*;

const NAME_SPACE: &str = &"plugin.common.map";

pub fn load() -> impl PluginRegistry {
    plugin(
        NAME_SPACE,
        "typed",
        factory!(typed, TypedConfig, new_typed_context),
    )
    .service(
        "mapping",
        factory!(mapping, MappingConfig, new_mapping_context),
    )
    .service("naming", factory!(naming, NamingConfig, new_naming_context))
    .service(
        "indexing",
        factory!(indexing, IndexingConfig, new_indexing_context),
    )
    .service(
        "reorder",
        factory!(reorder, ReorderConfig, new_reorder_context),
    )
}
