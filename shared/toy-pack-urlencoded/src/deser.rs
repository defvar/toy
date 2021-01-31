use crate::deserializer::Parse;
use crate::error::QueryParseError;
use toy_pack::deser::Deserializable;

/// Deserialize from byte slice.
///
/// # Example
///
/// ```
/// # use toy_pack_derive::*;
///
/// #[derive(Debug, PartialEq, Unpack)]
/// struct User {
///   id: u32,
///   name: String
/// }
///
/// fn main() {
///   let bytes: &[u8] = "id=123&name=aiueo".as_bytes();
///   let user = toy_pack_urlencoded::unpack::<User>(bytes).unwrap();
///
///   assert_eq!(user, User { id: 123, name: "aiueo".to_string() })
/// }
///
/// ```
#[inline]
pub fn unpack<'toy, T>(slice: &'toy [u8]) -> Result<T, QueryParseError>
where
    T: Deserializable<'toy>,
{
    T::deserialize(Parse::new(slice))
}
