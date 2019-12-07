//! Decoder for MessagePack data.

use std::{io, result};

pub use self::decoder::Decoder;
pub use self::decoder_ops::{DecoderOps, Reference};
pub use self::error::DecodeError;
pub use self::reader::Reader;

mod decoder_ops;
mod reader;
mod decoder;
mod error;

pub type Result<T> = result::Result<T, DecodeError>;

/// Create decoder from byte slice.
///
/// # Example
///
/// ```edition2018
/// use toy_pack_mp::{decoder_from_slice, DecoderOps};
///
/// let vec: Vec<u8> = vec![0xcd as u8, 0, 0];
///
/// let mut decoder = decoder_from_slice(&vec[..]);
///
/// assert_eq!(0u16, decoder.decode_integer().unwrap());
///
/// ```
pub fn decoder_from_slice(slice: &[u8]) -> Decoder<reader::SliceReader> {
    Decoder::new(reader::SliceReader::new(slice))
}

/// Create decoder from [`io::Read`].
///
/// [`io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
///
/// # Example
///
/// ```edition2018
/// use std::io::BufReader;
///
/// use toy_pack_mp::{decoder_from_reader, DecoderOps};
///
/// let mut bytes: &[u8] = &[0xcd as u8, 0, 0];
/// let reader = BufReader::new(bytes);
///
/// let mut decoder = decoder_from_reader(reader);
///
/// assert_eq!(0u16, decoder.decode_integer().unwrap());
///
/// ```
pub fn decoder_from_reader<R: io::Read>(reader: R) -> Decoder<reader::IoReader<R>> {
    Decoder::new(reader::IoReader::new(reader))
}
