use crate::decoder::Decoder;
use crate::error::YamlError;
use toy_pack::deser::Deserializable;

#[inline]
pub fn unpack<'toy, T>(s: &'toy str) -> Result<T, YamlError>
where
    T: Deserializable<'toy>,
{
    match Decoder::from_str(s) {
        Ok(mut d) => T::deserialize(&mut d),
        Err(e) => Err(e),
    }
}
