use super::service::*;
use toy_core::prelude::{Layered, NoopEntry};
use toy_core::registry::layer;

const NAME_SPACE: &str = &"plugin.common.stat";

pub fn cpu() -> (&'static str, &'static str, Cpu) {
    (NAME_SPACE, "cpu", Cpu)
}

pub fn memory() -> (&'static str, &'static str, Memory) {
    (NAME_SPACE, "memory", Memory)
}

pub fn all() -> Layered<Layered<NoopEntry, Cpu>, Memory> {
    layer(cpu()).layer(memory())
}
