//! Toy Plugin for File read and write.

use toy_pack::{Schema, Unpack};

use self::file_reader::FileReaderState;
pub use self::file_reader::{FileReader, RowIntoIterator, RowIterator};
pub use self::file_reader_builder::FileReaderBuilder;
pub use self::file_writer::FileWriter;
pub use self::file_writer_builder::FileWriterBuilder;
pub use plugin::load;

pub mod config;
mod file_reader;
mod file_reader_builder;
mod file_writer;
mod file_writer_builder;
mod plugin;
pub mod service;

#[derive(Clone, Copy, Debug, Unpack, Schema)]
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
