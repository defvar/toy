use std::io;

use toy_pack::deser::{Deserializable, DeserializableOwned};

use super::decode::{decoder_from_reader, decoder_from_slice, DecodeError};

#[inline]
pub fn unpack<'toy, T>(slice: &'toy [u8]) -> Result<T, DecodeError>
where
    T: Deserializable<'toy>,
{
    T::deserialize(&mut decoder_from_slice(slice))
}

#[inline]
pub fn unpack_from_reader<R, T>(reader: R) -> Result<T, DecodeError>
where
    R: io::Read,
    T: DeserializableOwned,
{
    T::deserialize(&mut decoder_from_reader(reader))
}
