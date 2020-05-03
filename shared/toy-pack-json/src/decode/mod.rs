use core::ops::Deref;
use std::io;

mod decoder;
mod error;
mod reader;
mod token;

pub use self::decoder::Decoder;
pub use self::error::DecodeError;
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

pub fn decoder_from_slice(slice: &[u8]) -> Decoder<reader::SliceReader> {
    Decoder::new(reader::SliceReader::new(slice))
}

pub fn decoder_from_reader<R: io::Read>(reader: R) -> Decoder<reader::IoReader<R>> {
    Decoder::new(reader::IoReader::new(reader))
}
