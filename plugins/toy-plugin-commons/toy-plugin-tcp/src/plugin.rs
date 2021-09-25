use crate::service::*;

const NAME_SPACE: &str = &"plugin.common.tcp";

pub fn write() -> (&'static str, &'static str, TcpWrite) {
    (NAME_SPACE, "write", TcpWrite)
}
