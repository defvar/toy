//! Decoder for Json data.

use core::ops::Deref;
use std::io;

mod decoder;
mod error;
mod reader;
mod token;

pub use self::decoder::Decoder;
pub use self::error::{DecodeError, DecodeErrorKind};
pub use self::reader::Position;
pub use self::reader::{IoReader, Reader, SliceReader};
pub use self::token::Token;

pub type Result<T> = std::result::Result<T, DecodeError>;

pub enum Reference<'b, 'c, T: ?Sized + 'static> {
    Borrowed(&'b T),
    Copied(&'c T),
}

impl<'b, 'c, T: ?Sized + 'static> Deref for Reference<'b, 'c, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseNumber {
    U64(u64),
    I64(i64),
    F64(f64),
}

pub fn decoder_from_slice(slice: &[u8]) -> Decoder<reader::SliceReader> {
    Decoder::new(reader::SliceReader::new(slice))
}

pub fn decoder_from_reader<R: io::Read>(reader: R) -> Decoder<reader::IoReader<R>> {
    Decoder::new(reader::IoReader::new(reader))
}
