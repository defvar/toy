use super::service::*;

const NAME_SPACE: &str = &"plugin.common.map";

pub fn mapping() -> (&'static str, &'static str, Mapping) {
    (NAME_SPACE, "mapping", Mapping)
}

pub fn indexing() -> (&'static str, &'static str, Indexing) {
    (NAME_SPACE, "indexing", Indexing)
}

pub fn reindexing() -> (&'static str, &'static str, Reindexing) {
    (NAME_SPACE, "reindexing", Reindexing)
}

pub fn naming() -> (&'static str, &'static str, Naming) {
    (NAME_SPACE, "naming", Naming)
}

pub fn rename() -> (&'static str, &'static str, Rename) {
    (NAME_SPACE, "rename", Rename)
}

pub fn put() -> (&'static str, &'static str, Put) {
    (NAME_SPACE, "put", Put)
}

pub fn remove_by_index() -> (&'static str, &'static str, RemoveByIndex) {
    (NAME_SPACE, "removeByIndex", RemoveByIndex)
}

pub fn remove_by_name() -> (&'static str, &'static str, RemoveByName) {
    (NAME_SPACE, "removeByName", RemoveByName)
}

pub fn single_value() -> (&'static str, &'static str, SingleValue) {
    (NAME_SPACE, "singleValue", SingleValue)
}

pub fn to_map() -> (&'static str, &'static str, ToMap) {
    (NAME_SPACE, "toMap", ToMap)
}

pub fn to_seq() -> (&'static str, &'static str, ToSeq) {
    (NAME_SPACE, "toSeq", ToSeq)
}
