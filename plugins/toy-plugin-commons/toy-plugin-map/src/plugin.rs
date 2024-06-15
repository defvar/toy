use super::transform_service::*;
use toy_core::prelude::{layer, Layered, NoopEntry};
use crate::typed::Typed;

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

pub fn typed() -> (&'static str, &'static str, Typed) {
    (NAME_SPACE, "typed", Typed)
}

pub fn all() -> Layered<
    Layered<
        Layered<
            Layered<
                Layered<
                    Layered<
                        Layered<
                            Layered<
                                Layered<
                                    Layered<Layered<Layered<NoopEntry, Mapping>, Indexing>, Reindexing>,
                                    Naming,
                                >,
                                Rename,
                            >,
                            Put,
                        >,
                        RemoveByIndex,
                    >,
                    RemoveByName,
                >,
                SingleValue,
            >,
            ToMap,
        >,
        ToSeq,
    >,
    Typed> {
    layer(mapping())
        .layer(indexing())
        .layer(reindexing())
        .layer(naming())
        .layer(rename())
        .layer(put())
        .layer(remove_by_index())
        .layer(remove_by_name())
        .layer(single_value())
        .layer(to_map())
        .layer(to_seq())
        .layer(typed())
}
