use crate::error::YamlError;
use serde::de::DeserializeOwned;

pub fn unpack<T>(s: &str) -> Result<T, YamlError>
where
    T: DeserializeOwned,
{
    serde_yaml::from_str(s).map_err(|e| YamlError::error(e))
}
