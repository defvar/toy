use crate::error::QueryParseError;
use serde::Serialize;

/// Serialize from data structure to `String`.
///
/// # Example
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///   id: u32,
///   name: String
/// }
///
/// fn main() {
///   let u = User {
///     id: 1,
///     name: "aiueo".to_string(),
///   };
///
///   let q = toy_pack_urlencoded::pack_to_string(&u).unwrap();
///   assert_eq!(q, "id=1&name=aiueo");
/// }
///
/// ```
#[inline]
pub fn pack_to_string<T>(item: &T) -> Result<String, QueryParseError>
where
    T: Serialize,
{
    serde_urlencoded::to_string(item).map_err(|e| QueryParseError::custom(e))
}
