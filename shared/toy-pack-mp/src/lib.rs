//! Implementation for MessagePack

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

pub use self::decode::{
    DecodeError,
    Decoder,
    decoder_from_reader,
    decoder_from_slice,
    DecoderOps,
};
pub use self::deser::{
    unpack,
    unpack_from_reader,
};
pub use self::encode::{
    EncodeError,
    Encoder,
    encoder_from_writer,
    EncoderOps,
};
pub use self::ser::{
    pack,
    pack_to_writer,
};

mod decode;
mod encode;
pub mod marker;
mod deser_ops;
mod ser_ops;

mod deserializer;
mod serializer;
mod ser;
mod deser;
