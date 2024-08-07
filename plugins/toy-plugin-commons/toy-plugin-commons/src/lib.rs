//! Toy Plugins.

pub mod map {
    pub use toy_plugin_map::*;
}

pub mod buffer {
    pub use toy_plugin_buffer::*;
}

pub mod collect {
    pub use toy_plugin_collect::*;
}

pub mod fanout {
    pub use toy_plugin_fanout::*;
}

pub mod stdio {
    pub use toy_plugin_stdio::*;
}

pub mod file {
    pub use toy_plugin_file::*;
}

pub mod tcp {
    pub use toy_plugin_tcp::*;
}

pub mod timer {
    pub use toy_plugin_timer::*;
}

pub mod sort {
    pub use toy_plugin_sort::*;
}

pub mod filter {
    pub use toy_plugin_filter::*;
}

pub mod stat {
    pub use toy_plugin_stat::*;
}