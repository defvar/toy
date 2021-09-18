//! Toy Plugin for File read and write.

#![feature(type_alias_impl_trait)]

use serde::{Deserialize, Serialize};
use toy_pack::Schema;

pub use plugin::{read, write};
pub mod config;
pub mod file_reader;
pub mod file_reader_builder;
pub mod file_writer;
pub mod file_writer_builder;
mod plugin;
pub mod service;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Schema)]
pub enum QuoteStyle {
    Always,
    Necessary,
    Never,
}

impl Default for QuoteStyle {
    fn default() -> QuoteStyle {
        QuoteStyle::Necessary
    }
}
