use std::io;

use toy_pack::ser::Serializable;

use super::encode::{encoder_from_writer, EncodeError};

#[inline]
pub fn pack<T>(item: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serializable,
{
    let mut writer = Vec::with_capacity(128);
    pack_to_writer(&mut writer, item)?;
    Ok(writer)
}

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
