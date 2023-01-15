use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

pub struct Drop<'a> {
    columns: Option<Vec<&'a str>>,
}

impl<'a> Drop<'a> {
    pub fn none() -> Drop<'a> {
        Drop { columns: None }
    }

    pub fn with(columns: Vec<&'a str>) -> Drop<'a> {
        Drop {
            columns: Some(columns),
        }
    }

    pub fn new() -> Drop<'a> {
        Drop { columns: None }
    }

    pub fn push(mut self, column: &'a str) -> Drop<'a> {
        self.columns.as_mut().map(|x| x.push(column));
        self
    }
}

impl<'a> FluxPart for Drop<'a> {
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = 0;
        r += writer.write(&b"drop("[..])?;

        if let Some(columns) = &self.columns {
            r += writer.write(&b"columns: ["[..])?;
            r += columns
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

            r += writer.write(&[b']'])?;
        }

        r += writer.write(&[b')'])?;

        Ok(r)
    }
}
