use crate::transform::{PutValue, Transform};
use std::collections::HashMap;
use toy_pack_derive::*;

/*

# transform vec from map value.
indexed = ["id", "name", "age"]

# reorder element for vec.
# drop unused index.
reorder = [5, 4, 3]

# transform map from vec value.
named = { id = 5, name = 4, age = 3 }

# if value is seq then put by tail.
[put]
aaa = { value = "put aaa", tp = "str" }
event_time = { tp = "$event_time$" } }

# remove element by index.
remove_by_index = [1,2]

# remove element by name.
remove_by_name = ["age"]

*/

/*

# cast as "type_name"
[typed]
id = { tp = "u32" }
name = { tp = "String", default_value = "aiueo" }
age = { tp = "u32" }
"child.id" = { tp = "u32" }

*/

#[derive(Debug, Clone, Default, UnPack)]
pub struct TypedConfig {
    pub typed: HashMap<String, TypedConfigOption>,
}

#[derive(Debug, Clone, Default, UnPack)]
pub struct TypedConfigOption {
    pub tp: String,
    pub default_value: Option<String>,
}

pub trait ToTransform {
    fn into_transform(self) -> Option<Transform>;
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

impl ToTransform for IndexedConfig {
    fn into_transform(self) -> Option<Transform> {
        self.indexed.map(Transform::Indexed)
    }
}

impl ToTransform for ReorderConfig {
    fn into_transform(self) -> Option<Transform> {
        self.reorder.map(Transform::Reorder)
    }
}

impl ToTransform for NamedConfig {
    fn into_transform(self) -> Option<Transform> {
        self.named.map(Transform::Named)
    }
}

impl ToTransform for PutConfig {
    fn into_transform(self) -> Option<Transform> {
        self.put.map(Transform::Put)
    }
}

impl ToTransform for RemoveByIndexConfig {
    fn into_transform(self) -> Option<Transform> {
        self.remove_by_index
            .map(|mut x| {
                x.sort();
                x.reverse();
                x
            })
            .map(Transform::RemoveByIndex)
    }
}

impl ToTransform for RemoveByNameConfig {
    fn into_transform(self) -> Option<Transform> {
        self.remove_by_name.map(Transform::RemoveByName)
    }
}
