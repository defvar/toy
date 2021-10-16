use super::service::*;
use toy_core::prelude::{layer, Layered, NoopEntry};

const NAME_SPACE: &str = &"plugin.common.stdio";

pub fn stdin() -> (&'static str, &'static str, Stdin) {
    (NAME_SPACE, "stdin", Stdin)
}

pub fn stdout() -> (&'static str, &'static str, Stdout) {
    (NAME_SPACE, "stdout", Stdout)
}

pub fn all() -> Layered<Layered<NoopEntry, Stdin>, Stdout> {
    layer(stdin()).layer(stdout())
}
