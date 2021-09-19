//! Toy Plugins.

pub use toy_plugin_collect::{count, first, last};
pub use toy_plugin_fanout::broadcast;
pub use toy_plugin_file::{read, write};
pub use toy_plugin_map::{
    indexing, mapping, naming, put, reindexing, remove_by_index, remove_by_name, rename,
    single_value, to_map, to_seq,
};
pub use toy_plugin_stdio::{stdin, stdout};
pub use toy_plugin_timer::tick;
