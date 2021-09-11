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
        "reindexing",
        PortType::flow(),
        factory!(reindexing, ReindexingConfig, new_reindexing_context),
    )
    .with(
        "rename",
        PortType::flow(),
        factory!(rename, RenameConfig, new_rename_context),
    )
    .with(
        "removeByIndex",
        PortType::flow(),
        factory!(
            remove_by_index,
            RemoveByIndexConfig,
            new_remove_by_index_context
        ),
    )
    .with(
        "removeByName",
        PortType::flow(),
        factory!(
            remove_by_name,
            RemoveByNameConfig,
            new_remove_by_name_context
        ),
    )
    .with(
        "put",
        PortType::flow(),
        factory!(put, PutConfig, new_put_context),
    )
    .with(
        "singleValue",
        PortType::flow(),
        factory!(single_value, SingleValueConfig, new_single_value_context),
    )
    .with(
        "toMap",
        PortType::flow(),
        factory!(to_map, ToMapConfig, new_to_map_context),
    )
    .with(
        "toSeq",
        PortType::flow(),
        factory!(to_seq, ToSeqConfig, new_to_seq_context),
    )
}
