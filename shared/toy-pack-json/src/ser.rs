use std::io;

use toy_pack::ser::Serializable;

use super::encode::{encoder_from_writer, EncodeError};
use crate::encode::encoder_from_writer_pretty;

/// Serialize from data structure to [`Vec<u8>`].
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
/// # Example
///
/// ```edition2018
/// use toy_pack_derive::*;
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
///     name: "a".to_string(),
///   };
///
///   toy_pack_json::pack(&u).unwrap();
/// }
///
/// ```
#[inline]
pub fn pack<T>(item: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serializable,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer(&mut writer, item)?;
    Ok(writer)
}

/// Serialize from data structure as pretty-printed to [`Vec<u8>`].
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
#[inline]
pub fn pack_pretty<T>(item: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serializable,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer_pretty(&mut writer, item)?;
    Ok(writer)
}

/// Serialize from data structure to `String`.
///
#[inline]
pub fn pack_to_string<T>(item: &T) -> Result<String, EncodeError>
where
    T: Serializable,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer(&mut writer, item)?;
    Ok(unsafe { String::from_utf8_unchecked(writer) })
}

/// Serialize from data structure as pretty-printed to `String`.
///
#[inline]
pub fn pack_to_string_pretty<T>(item: &T) -> Result<String, EncodeError>
where
    T: Serializable,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer_pretty(&mut writer, item)?;
    Ok(unsafe { String::from_utf8_unchecked(writer) })
}

/// Serialize from data structure to [`io::Write`].
///
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
///
#[inline]
pub fn pack_to_writer<W, T>(writer: W, item: &T) -> Result<(), EncodeError>
where
    W: io::Write,
    T: Serializable,
{
    let mut w = encoder_from_writer(writer);
    item.serialize(&mut w)?;
    Ok(())
}

/// Serialize from data structure as pretty-printed to [`io::Write`].
///
/// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
///
#[inline]
pub fn pack_to_writer_pretty<W, T>(writer: W, item: &T) -> Result<(), EncodeError>
where
    W: io::Write,
    T: Serializable,
{
    let mut w = encoder_from_writer_pretty(writer);
    item.serialize(&mut w)?;
    Ok(())
}
