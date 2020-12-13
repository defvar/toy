//! Toy Plugin for Transform Object.

pub mod config;
mod plugin;
pub mod service;
mod transform;
mod typed;

pub use self::transform::{
    Indexing, Mapping, NameOrIndex, Naming, Put, Reindexing, RemoveByIndex, RemoveByName, Rename,
    SingleValue,
};
pub use self::transform::{PutValue, Transformer};
pub use plugin::load;
pub use typed::{convert, AllowedTypes};
