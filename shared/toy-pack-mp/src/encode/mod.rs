//! Encoder for MessagePack data.

use std::{io, result};

pub use self::encoder::Encoder;
pub use self::encoder_ops::EncoderOps;
pub use self::error::EncodeError;
pub use self::writer::Writer;

mod encoder;
mod encoder_ops;
mod error;
mod writer;

pub type Result<T> = result::Result<T, EncodeError>;

/// Create encoder from [`io::Write`].
///
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
///
/// # Example
///
/// ```edition2018
/// use toy_pack_mp::{encoder_from_writer, EncoderOps};
///
/// let mut vec: Vec<u8> = Vec::new();
/// let mut encoder = encoder_from_writer(&mut vec);
/// encoder.encode_uint(1).unwrap();
///
/// ```
///
pub fn encoder_from_writer<W: io::Write>(writer: W) -> Encoder<writer::IoWriter<W>> {
    Encoder::new(writer::IoWriter::new(writer))
}

/// Create encoder from `Vec<u8>`.
///
/// # Example
///
/// ```edition2018
/// use toy_pack_mp::{encoder_from_vec, EncoderOps};
///
/// let mut vec: Vec<u8> = Vec::new();
/// let mut encoder = encoder_from_vec(&mut vec);
/// encoder.encode_uint(1).unwrap();
///
/// ```
///
pub fn encoder_from_vec(slice: &'_ mut Vec<u8>) -> Encoder<writer::VecWriter<'_>> {
    Encoder::new(writer::VecWriter::new(slice))
}
