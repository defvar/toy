use std::io;

use serde::Serialize;

use super::encode::{encoder_from_writer, EncodeError};

/// Serialize from data structure to [`Vec<u8>`].
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
/// # Example
///
/// ```edition2018
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
///     name: "a".to_string(),
///   };
///
///   toy_pack_mp::pack(&u).unwrap();
/// }
///
/// ```
#[inline]
pub fn pack<T>(item: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer(&mut writer, item)?;
    Ok(writer)
}

/// Serialize from data structure to [`io::Write`].
///
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
///
#[inline]
pub fn pack_to_writer<W, T>(writer: W, item: &T) -> Result<(), EncodeError>
where
    W: io::Write,
    T: Serialize,
{
    let mut w = encoder_from_writer(writer);
    item.serialize(&mut w)?;
    Ok(())
}
