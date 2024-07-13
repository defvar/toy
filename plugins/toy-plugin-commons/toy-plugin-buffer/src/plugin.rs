use toy_core::prelude::{layer, Layered, NoopEntry};
use super::service::*;

const NAME_SPACE: &str = &"plugin.common.buffer";

pub fn fixed_size() -> (&'static str, &'static str, FixedSize) {
    (NAME_SPACE, "fixedSize", FixedSize)
}

pub fn all() -> Layered<NoopEntry, FixedSize> {
    layer(fixed_size())
}
