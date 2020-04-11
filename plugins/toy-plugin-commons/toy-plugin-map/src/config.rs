use crate::transform::{
    Indexed, Named, Put, PutValue, RemoveByIndex, RemoveByName, Rename, Reorder, Transformer,
};
use std::collections::HashMap;
use toy_pack_derive::*;

/// config for type convert.
///
#[derive(Debug, Clone, Default, UnPack)]
pub struct TypedConfig {
    /// key: field name, value: option
    pub typed: HashMap<String, TypedConfigOption>,
}

/// config detail for type convert.
///
#[derive(Debug, Clone, Default, UnPack)]
pub struct TypedConfigOption {
    /// type name string
    /// e.g.) "u32"
    pub tp: String,

    /// default value string
    /// e.g.) "123"
    pub default_value: Option<String>,
}

pub trait ToTransform<T>
where
    T: Transformer,
{
    fn into_transform(self) -> Option<T>;
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct IndexedConfig {
    pub indexed: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct ReorderConfig {
    pub reorder: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct NamedConfig {
    pub named: Option<HashMap<String, u32>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct RenameConfig {
    pub rename: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct PutConfig {
    pub put: Option<HashMap<String, PutValue>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct RemoveByIndexConfig {
    pub remove_by_index: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct RemoveByNameConfig {
    pub remove_by_name: Option<Vec<String>>,
}

impl ToTransform<Indexed> for IndexedConfig {
    fn into_transform(self) -> Option<Indexed> {
        self.indexed.map(Indexed)
    }
}

impl ToTransform<Reorder> for ReorderConfig {
    fn into_transform(self) -> Option<Reorder> {
        self.reorder.map(Reorder)
    }
}

impl ToTransform<Named> for NamedConfig {
    fn into_transform(self) -> Option<Named> {
        self.named.map(Named)
    }
}

impl ToTransform<Rename> for RenameConfig {
    fn into_transform(self) -> Option<Rename> {
        self.rename.map(Rename)
    }
}

impl ToTransform<Put> for PutConfig {
    fn into_transform(self) -> Option<Put> {
        self.put.map(Put)
    }
}

impl ToTransform<RemoveByIndex> for RemoveByIndexConfig {
    fn into_transform(self) -> Option<RemoveByIndex> {
        self.remove_by_index
            .map(|mut x| {
                x.sort();
                x.reverse();
                x
            })
            .map(RemoveByIndex)
    }
}

impl ToTransform<RemoveByName> for RemoveByNameConfig {
    fn into_transform(self) -> Option<RemoveByName> {
        self.remove_by_name.map(RemoveByName)
    }
}
