use crate::error::QueryParseError;
use crate::serializer::Encoder;
use toy_pack::ser::Serializable;

/// Serialize from data structure to `String`.
///
/// # Example
///
/// ```
/// # use toy_pack_derive::*;
///
/// #[derive(Pack)]
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
    T: Serializable,
{
    let mut urlencoder = form_urlencoded::Serializer::new("".to_owned());
    item.serialize(Encoder::new(&mut urlencoder))?;
    Ok(urlencoder.finish())
}
