//! Toy Plugin for File read and write.

#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use serde::{Deserialize, Serialize};
use toy_pack::Schema;

pub use file_reader_builder::FileReaderBuilder;
pub use file_writer_builder::FileWriterBuilder;
pub use plugin::{all, read, write};

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
