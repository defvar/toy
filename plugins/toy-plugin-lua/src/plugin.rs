use super::service::*;

const NAME_SPACE: &str = &"plugin.lua";

pub fn lua() -> (&'static str, &'static str, LuaFunction) {
    (NAME_SPACE, "Function", LuaFunction)
}
