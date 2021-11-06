use crate::service::Sort;
use toy_core::prelude::{layer, Layered, NoopEntry};

const NAME_SPACE: &str = &"plugin.common.sort";

pub fn sort() -> (&'static str, &'static str, Sort) {
    (NAME_SPACE, "sort", Sort)
}

pub fn all() -> Layered<NoopEntry, Sort> {
    layer(sort())
}
