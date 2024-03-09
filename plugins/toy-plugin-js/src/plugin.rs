use super::service::*;

const NAME_SPACE: &str = &"plugin.js";

pub fn js() -> (&'static str, &'static str, Function) {
    (NAME_SPACE, "Function", Function)
}
