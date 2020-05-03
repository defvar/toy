#[macro_use]
extern crate failure;

pub use self::de::{unpack, unpack_from_reader};
pub use self::decode::{
    decoder_from_reader, decoder_from_slice, DecodeError, DecodeErrorKind, Decoder, ParseNumber,
};
pub use self::encode::{encoder_from_writer, EncodeError, EncodeErrorKind, Encoder};
pub use self::ser::{pack, pack_to_writer};

mod de;
mod decode;
mod deser_ops;
mod deserializer;
mod encode;
mod ser;
mod ser_ops;
mod serializer;
