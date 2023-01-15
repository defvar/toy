use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;
use toy_map::Map;

pub struct Rename<'a> {
    map: Map<&'a str, &'a str>,
}

impl<'a> Rename<'a> {
    pub fn with(map: &[(&'a str, &'a str)]) -> Rename<'a> {
        Rename {
            map: map.to_vec().into_iter().collect(),
        }
    }
}

impl<'a> FluxPart for Rename<'a> {
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = 0;
        r += writer.write(&b"rename(columns: {"[..])?;
        self.map
            .iter()
            .enumerate()
            .try_fold(0, |mut acc, (index, (k, v))| {
                if index != 0 {
                    acc += writer.write(&[b',', b' '])?;
                }
                acc += writer.write(k.as_bytes())?;
                acc += writer.write(&[b':'])?;
                acc += writer.write(&[b'\"'])?;
                acc += writer.write(v.as_bytes())?;
                acc += writer.write(&[b'\"'])?;
                Ok::<usize, InfluxDBError>(acc)
            })?;
        r += writer.write(&[b'}', b')'])?;
        Ok(r)
    }
}
