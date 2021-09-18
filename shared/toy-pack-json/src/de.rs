use std::io;

use serde::de::{Deserialize, DeserializeOwned};

use super::decode::{decoder_from_reader, decoder_from_slice, DecodeError};

/// Deserialize from byte slice.
///
/// # Example
///
/// ```edition2018
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct User {
///   id: u32,
///   name: String
/// }
///
/// fn main() {
///   let json = r#"
///   {
///     "id": 123,
///     "name": "taro"
///   }"#;
///   toy_pack_json::unpack::<User>(json.as_bytes()).unwrap();
/// }
///
/// ```
#[inline]
pub fn unpack<'toy, T>(slice: &'toy [u8]) -> Result<T, DecodeError>
where
    T: Deserialize<'toy>,
{
    T::deserialize(&mut decoder_from_slice(slice))
}

/// Deserialize from [`io::Read`].
///
/// [`io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
#[inline]
pub fn unpack_from_reader<R, T>(reader: R) -> Result<T, DecodeError>
where
    R: io::Read,
    T: DeserializeOwned,
{
    T::deserialize(&mut decoder_from_reader(reader))
}
