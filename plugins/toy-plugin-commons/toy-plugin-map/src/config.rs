use crate::transform::{
    Indexing, Mapping, NameOrIndex, Naming, Put, PutValue, Reindexing, RemoveByIndex, RemoveByName,
    Rename, SingleValue, Transformer,
};
use crate::typed::AllowedTypes;
use std::collections::HashMap;
use toy_core::data::Map;
use toy_pack::{Schema, Unpack};

/// config for type convert.
///
#[derive(Debug, Clone, Default, Unpack, Schema)]
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
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct TypedConfigOption {
    pub tp: AllowedTypes,

    /// default value string
    /// e.g.) "123"
    pub default_value: Option<String>,
}

/// transform to map value from map value.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct MappingConfig {
    /// key: filed name, value: new field name.
    pub mappings: Map<String, String>,
}

/// transform to seq value from map value.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct IndexingConfig {
    pub names: Vec<String>,
}

/// reindexing element for seq value.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct ReindexingConfig {
    pub reindexing: Vec<u32>,
}

/// transform to map value from seq value.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct NamingConfig {
    /// key: filed name, value: seq index.
    pub names: HashMap<String, u32>,
}

/// rename field for map value.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct RenameConfig {
    pub rename: HashMap<String, String>,
}

/// put field or element for map or seq.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct PutConfig {
    pub put: HashMap<String, PutValue>,
}

/// remove field by Index.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct RemoveByIndexConfig {
    pub remove_by_index: Vec<u32>,
}

/// remove field by name.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct RemoveByNameConfig {
    pub remove_by_name: Vec<String>,
}

/// create single value from map or seq.
#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct SingleValueConfig {
    pub name_or_index: NameOrIndex,
}

pub trait ToTransform<T>
where
    T: Transformer,
{
    fn into_transform(self) -> T;
}

impl ToTransform<Mapping> for MappingConfig {
    fn into_transform(self) -> Mapping {
        Mapping(self.mappings)
    }
}

impl ToTransform<Indexing> for IndexingConfig {
    fn into_transform(self) -> Indexing {
        Indexing(self.names)
    }
}

impl ToTransform<Reindexing> for ReindexingConfig {
    fn into_transform(self) -> Reindexing {
        Reindexing(self.reindexing)
    }
}

impl ToTransform<Naming> for NamingConfig {
    fn into_transform(self) -> Naming {
        Naming(self.names)
    }
}

impl ToTransform<Rename> for RenameConfig {
    fn into_transform(self) -> Rename {
        Rename(self.rename)
    }
}

impl ToTransform<Put> for PutConfig {
    fn into_transform(self) -> Put {
        Put(self.put)
    }
}

impl ToTransform<RemoveByIndex> for RemoveByIndexConfig {
    fn into_transform(mut self) -> RemoveByIndex {
        self.remove_by_index.sort();
        self.remove_by_index.reverse();
        RemoveByIndex(self.remove_by_index)
    }
}

impl ToTransform<RemoveByName> for RemoveByNameConfig {
    fn into_transform(self) -> RemoveByName {
        RemoveByName(self.remove_by_name)
    }
}

impl ToTransform<SingleValue> for SingleValueConfig {
    fn into_transform(self) -> SingleValue {
        SingleValue(self.name_or_index)
    }
}
