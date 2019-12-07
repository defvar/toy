#[macro_use]
extern crate failure;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use error::ConfigError;
use toy_pack::deser::DeserializableOwned;
use toy_pack_yaml::Decoder;

pub mod error;
pub mod utils;

pub trait Config {
    fn validate(&self) -> Result<(), ConfigError>;
}

pub fn from_yaml<'a, T, P>(path: P) -> Result<T, ConfigError>
where
    T: Config + DeserializableOwned<Value = T>,
    P: AsRef<Path>,
{
    let mut buf: Vec<u8> = Vec::new();
    let mut f = File::open(path)?;
    f.read_to_end(&mut buf)?;
    let text = std::str::from_utf8(&buf[..])?;
    let mut yml = Decoder::from_str(text)?;

    T::deserialize(&mut yml).map_err(Into::into)
}
