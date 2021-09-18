use crate::error::YamlError;
use serde::Serialize;

pub fn pack_to_string<T>(v: T) -> Result<String, YamlError>
where
    T: Serialize,
{
    serde_yaml::to_string(&v).map_err(|e| YamlError::error(e))
}
