use super::service::*;

const NAME_SPACE: &str = &"plugin.common.timer";

pub fn tick() -> (&'static str, &'static str, Tick) {
    (NAME_SPACE, "tick", Tick)
}
