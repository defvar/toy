use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

pub struct Sort<'a> {
    columns: Vec<&'a str>,
}

impl<'a> Sort<'a> {
    pub fn with(columns: Vec<&'a str>) -> Sort<'a> {
        Sort { columns }
    }

    pub fn new() -> Sort<'a> {
        Sort {
            columns: Vec::new(),
        }
    }

    pub fn push(mut self, column: &'a str) -> Sort<'a> {
        self.columns.push(column);
        self
    }
}

impl<'a> FluxPart for Sort<'a> {
    fn need(&self) -> bool {
        return self.columns.len() > 0;
    }

    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = 0;
        r += writer.write(&b"sort(columns: ["[..])?;

        r += self
            .columns
            .iter()
            .enumerate()
            .try_fold(0, |mut acc, (index, x)| {
                if index != 0 {
                    acc += writer.write(b",")?;
                }
                acc += writer.write(&[b'\"'])?;
                acc += writer.write(x.as_bytes())?;
                acc += writer.write(&[b'\"'])?;
                Ok::<usize, InfluxDBError>(acc)
            })?;

        r += writer.write(&[b']', b')'])?;
        Ok(r)
    }
}
