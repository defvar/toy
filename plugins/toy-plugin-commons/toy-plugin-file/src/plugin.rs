use super::service::*;

const NAME_SPACE: &str = &"plugin.common.file";

pub fn read() -> (&'static str, &'static str, Read) {
    (NAME_SPACE, "read", Read)
}

pub fn write() -> (&'static str, &'static str, Write) {
    (NAME_SPACE, "write", Write)
}
