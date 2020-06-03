//! Toy Plugin for File read and write.

use toy_pack::{Schema, Unpack};

use self::edges::Edges;
use self::file_reader::FileReaderState;
pub use self::file_reader::{FileReader, RowIntoIterator, RowIterator};
pub use self::file_reader_builder::FileReaderBuilder;
pub use self::file_writer::FileWriter;
pub use self::file_writer_builder::FileWriterBuilder;
pub use self::row::{ColumnIterator, Row};
pub use plugin::load;

pub mod config;
mod edges;
mod file_reader;
mod file_reader_builder;
mod file_writer;
mod file_writer_builder;
pub mod parse;
mod plugin;
mod row;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Unpack, Schema)]
pub enum Terminator {
    CRLF,
    Any(u8),
}

impl Terminator {
    pub fn is_crlf(&self) -> bool {
        match *self {
            Terminator::CRLF => true,
            Terminator::Any(_) => false,
        }
    }

    pub fn equals(&self, other: u8) -> bool {
        match *self {
            Terminator::CRLF => other == b'\r' || other == b'\n',
            Terminator::Any(b) => other == b,
        }
    }

    pub fn to_parse(&self) -> parse::Terminator {
        match *self {
            Terminator::CRLF => parse::Terminator::CRLF,
            Terminator::Any(b) => parse::Terminator::Any(b),
        }
    }
}

impl Default for Terminator {
    fn default() -> Terminator {
        Terminator::CRLF
    }
}
