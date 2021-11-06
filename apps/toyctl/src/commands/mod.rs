use crate::error::Error;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub mod list;
pub mod post;
pub mod put;

pub(crate) fn from_file<T>(file: PathBuf) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut f = File::open(file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let v = toy_pack_json::unpack(&buffer)?;
    Ok(v)
}
