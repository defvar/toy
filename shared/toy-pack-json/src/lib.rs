#[macro_use]
extern crate failure;

pub use self::de::{unpack, unpack_from_reader};
pub use self::decode::{decoder_from_reader, decoder_from_slice, DecodeError, Decoder};

mod de;
mod decode;
mod deser_ops;
mod deserializer;
