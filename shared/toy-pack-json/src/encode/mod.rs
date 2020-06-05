//! Encoder for Json data.

use std::{io, result};

pub use self::encoder::Encoder;
pub use self::error::EncodeError;

mod encoder;
mod error;

pub type Result<T> = result::Result<T, EncodeError>;

pub fn encoder_from_writer<W: io::Write>(writer: W) -> Encoder<W> {
    Encoder::new(writer)
}
