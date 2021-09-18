use crate::error::QueryParseError;
use serde::Deserialize;

/// Deserialize from byte slice.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
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
    T: Deserialize<'toy>,
{
    serde_urlencoded::from_bytes(slice).map_err(|e| QueryParseError::custom(e))
}
