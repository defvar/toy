use std::io;

use toy_pack::deser::{Deserializable, DeserializableOwned};

use super::decode::{decoder_from_reader, decoder_from_slice, DecodeError};

/// Deserialize from byte slice.
///
/// # Example
///
/// ```edition2018
/// use toy_pack_derive::*;
///
/// #[derive(Unpack)]
/// struct User {
///   id: u32,
///   name: String
/// }
///
/// fn main() {
///   let vec: Vec<u8> = vec![146, 206, 1, 0, 0, 0, 161, 97];
///   toy_pack_mp::unpack::<User>(&vec).unwrap();
/// }
///
/// ```
#[inline]
pub fn unpack<'toy, T>(slice: &'toy [u8]) -> Result<T, DecodeError>
where
    T: Deserializable<'toy>,
{
    T::deserialize(&mut decoder_from_slice(slice))
}

/// Deserialize from `io::Read`.
///
#[inline]
pub fn unpack_from_reader<R, T>(reader: R) -> Result<T, DecodeError>
where
    R: io::Read,
    T: DeserializableOwned,
{
    T::deserialize(&mut decoder_from_reader(reader))
}
