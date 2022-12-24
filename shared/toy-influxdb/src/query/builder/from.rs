use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct From<'a> {
    bucket: &'a str,
}

impl<'a> From<'a> {
    pub fn with(bucket: &'a str) -> From<'a> {
        Self { bucket }
    }
}

impl<'a> FluxPart for From<'a> {
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = writer.write(&b"from(bucket: \""[..])?;
        r += writer.write(self.bucket.as_bytes())?;
        r += writer.write(&[b'\"', b')'])?;
        Ok(r)
    }
}
