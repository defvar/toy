use crate::service::Filter;

const NAME_SPACE: &str = &"plugin.common.filter";

pub fn filter() -> (&'static str, &'static str, Filter) {
    (NAME_SPACE, "filter", Filter)
}
