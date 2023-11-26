//! # toy-pack Implementation for Json

#![feature(type_alias_impl_trait, error_generic_member_access)]

pub use self::de::{unpack, unpack_from_reader};
pub use self::decode::{
    decoder_from_reader, decoder_from_slice, DecodeError, Decoder, ParseNumber,
};
pub use self::encode::{encoder_from_writer, encoder_from_writer_pretty, EncodeError, Encoder};
pub use self::ser::{
    pack, pack_pretty, pack_to_string, pack_to_string_pretty, pack_to_writer, pack_to_writer_pretty,
};

mod de;
mod decode;
mod deser_ops;
mod deserializer;
mod encode;
pub mod jvalue;
mod ser;
mod ser_ops;
mod serializer;
