use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

pub struct Limit {
    n: usize,
    offset: usize,
}

impl Limit {
    pub fn with(n: usize, offset: usize) -> Limit {
        Limit { n, offset }
    }

    pub fn n(n: usize) -> Limit {
        Limit { n, offset: 0 }
    }
}

impl FluxPart for Limit {
    fn need(&self) -> bool {
        self.n > 0
    }

    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        if self.n == 0 {
            return Ok(0);
        }

        let mut r = writer.write(&b"limit(n: "[..])?;

        let mut buf = itoa::Buffer::new();
        let n = buf.format(self.n).as_bytes();

        r += writer.write(n)?;

        if self.offset > 0 {
            let offset = buf.format(self.offset).as_bytes();
            r += writer.write(&b", offset: "[..])?;
            r += writer.write(offset)?;
        }
        r += writer.write(&b")"[..])?;
        Ok(r)
    }
}
