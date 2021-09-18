use crate::transform::{
    IndexingTransformer, MappingTransformer, NameOrIndexTransformer, NamingTransformer,
    PutTransformer, PutValueTransformer, ReindexingTransformer, RemoveByIndexTransformer,
    RemoveByNameTransformer, RenameTransformer, SingleValueTransformer, ToMapTransformer,
    ToSeqTransformer, Transformer,
};
use crate::typed::AllowedTypes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toy_core::data::Map;
use toy_pack::Schema;

/// config for type convert.
///
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct TypedConfig {
    /// key: field name, value: option
    pub typed: HashMap<String, TypedConfigOption>,
}

/// config detail for type convert.
/// - **allowed types**
///   - bool
///   - u8 u16 u32 u64
///   - i8 i16 i32 i64
///   - f32 f64
///   - str
///
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct TypedConfigOption {
    pub tp: AllowedTypes,

    /// default value string
    /// e.g.) "123"
    pub default_value: Option<String>,
}

/// transform to map value from map value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct MappingConfig {
    /// key: filed name, value: new field name.
    pub mappings: Map<String, String>,
}

/// transform to seq value from map value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct IndexingConfig {
    pub names: Vec<String>,
}

/// reindexing element for seq value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct ReindexingConfig {
    pub reindexing: Vec<u32>,
}

/// transform to map value from seq value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct NamingConfig {
    /// key: filed name, value: seq index.
    pub names: HashMap<String, u32>,
}

/// rename field for map value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct RenameConfig {
    pub rename: HashMap<String, String>,
}

/// put field or element for map or seq.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct PutConfig {
    pub put: HashMap<String, PutValueTransformer>,
}

/// remove field by Index.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct RemoveByIndexConfig {
    pub remove_by_index: Vec<u32>,
}

/// remove field by name.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct RemoveByNameConfig {
    pub remove_by_name: Vec<String>,
}

/// create single value from map or seq.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct SingleValueConfig {
    pub name_or_index: NameOrIndexTransformer,
}

/// create single key and value from other value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct ToMapConfig {
    pub name: String,
}

/// create single element seq from other value.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct ToSeqConfig;

pub trait ToTransform<T>
where
    T: Transformer,
{
    fn into_transform(self) -> T;
}

impl ToTransform<MappingTransformer> for MappingConfig {
    fn into_transform(self) -> MappingTransformer {
        MappingTransformer(self.mappings)
    }
}

impl ToTransform<IndexingTransformer> for IndexingConfig {
    fn into_transform(self) -> IndexingTransformer {
        IndexingTransformer(self.names)
    }
}

impl ToTransform<ReindexingTransformer> for ReindexingConfig {
    fn into_transform(self) -> ReindexingTransformer {
        ReindexingTransformer(self.reindexing)
    }
}

impl ToTransform<NamingTransformer> for NamingConfig {
    fn into_transform(self) -> NamingTransformer {
        NamingTransformer(self.names)
    }
}

impl ToTransform<RenameTransformer> for RenameConfig {
    fn into_transform(self) -> RenameTransformer {
        RenameTransformer(self.rename)
    }
}

impl ToTransform<PutTransformer> for PutConfig {
    fn into_transform(self) -> PutTransformer {
        PutTransformer(self.put)
    }
}

impl ToTransform<RemoveByIndexTransformer> for RemoveByIndexConfig {
    fn into_transform(mut self) -> RemoveByIndexTransformer {
        self.remove_by_index.sort();
        self.remove_by_index.reverse();
        RemoveByIndexTransformer(self.remove_by_index)
    }
}

impl ToTransform<RemoveByNameTransformer> for RemoveByNameConfig {
    fn into_transform(self) -> RemoveByNameTransformer {
        RemoveByNameTransformer(self.remove_by_name)
    }
}

impl ToTransform<SingleValueTransformer> for SingleValueConfig {
    fn into_transform(self) -> SingleValueTransformer {
        SingleValueTransformer(self.name_or_index)
    }
}

impl ToTransform<ToMapTransformer> for ToMapConfig {
    fn into_transform(self) -> ToMapTransformer {
        ToMapTransformer(self.name)
    }
}

impl ToTransform<ToSeqTransformer> for ToSeqConfig {
    fn into_transform(self) -> ToSeqTransformer {
        ToSeqTransformer()
    }
}
