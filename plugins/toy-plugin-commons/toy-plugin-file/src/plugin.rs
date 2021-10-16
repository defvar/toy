use super::service::*;
use toy_core::prelude::{layer, Layered, NoopEntry};

const NAME_SPACE: &str = &"plugin.common.file";

pub fn read() -> (&'static str, &'static str, Read) {
    (NAME_SPACE, "read", Read)
}

pub fn write() -> (&'static str, &'static str, Write) {
    (NAME_SPACE, "write", Write)
}

pub fn all() -> Layered<Layered<NoopEntry, Read>, Write> {
    layer(read()).layer(write())
}
