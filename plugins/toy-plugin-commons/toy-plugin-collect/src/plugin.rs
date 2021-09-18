use super::service::*;

const NAME_SPACE: &str = &"plugin.common.collect";

pub fn first() -> (&'static str, &'static str, First) {
    (NAME_SPACE, "first", First)
}

pub fn last() -> (&'static str, &'static str, Last) {
    (NAME_SPACE, "last", Last)
}

pub fn count() -> (&'static str, &'static str, Count) {
    (NAME_SPACE, "count", Count)
}
