use super::service::*;
use toy_core::prelude::{Layered, NoopEntry};
use toy_core::registry::layer;

const NAME_SPACE: &str = &"plugin.common.timer";

pub fn tick() -> (&'static str, &'static str, Tick) {
    (NAME_SPACE, "tick", Tick)
}

pub fn all() -> Layered<NoopEntry, Tick> {
    layer(tick())
}
