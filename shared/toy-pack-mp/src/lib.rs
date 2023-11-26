//! # toy-pack Implementation for MessagePack

#![feature(type_alias_impl_trait, error_generic_member_access)]

#[macro_use]
extern crate lazy_static;

pub use self::decode::{decoder_from_reader, decoder_from_slice, DecodeError, Decoder, DecoderOps};
pub use self::deser::{unpack, unpack_from_reader};
pub use self::encode::{encoder_from_vec, encoder_from_writer, EncodeError, Encoder, EncoderOps};
pub use self::ser::{pack, pack_to_writer};

mod decode;
mod deser_ops;
mod encode;
pub mod marker;
mod ser_ops;

mod deser;
mod deserializer;
mod ser;
mod serializer;
