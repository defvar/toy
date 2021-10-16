use crate::service::*;
use toy_core::prelude::{layer, Layered, NoopEntry};

const NAME_SPACE: &str = &"plugin.common.tcp";

pub fn write() -> (&'static str, &'static str, TcpWrite) {
    (NAME_SPACE, "write", TcpWrite)
}

pub fn all() -> Layered<NoopEntry, TcpWrite> {
    layer(write())
}
