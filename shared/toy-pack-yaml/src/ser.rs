use crate::error::YamlError;
use crate::serializer::Ser;
use toy_pack::ser::Serializable;
use yaml_rust::YamlEmitter;

pub fn pack_to_string<T>(v: T) -> Result<String, YamlError>
where
    T: Serializable,
{
    let doc = v.serialize(Ser)?;
    let mut buf = String::new();
    YamlEmitter::new(&mut buf).dump(&doc)?;
    Ok(buf)
}
