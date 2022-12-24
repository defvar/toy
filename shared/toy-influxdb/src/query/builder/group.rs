use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

pub struct Group<'a> {
    columns: Option<Vec<&'a str>>,
}

impl<'a> Group<'a> {
    pub fn ungroup() -> Group<'a> {
        Group { columns: None }
    }

    pub fn with(columns: Vec<&'a str>) -> Group<'a> {
        Group {
            columns: Some(columns),
        }
    }

    pub fn new() -> Group<'a> {
        Group { columns: None }
    }

    pub fn push(mut self, column: &'a str) -> Group<'a> {
        self.columns.as_mut().map(|x| x.push(column));
        self
    }
}

impl<'a> FluxPart for Group<'a> {
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = 0;
        r += writer.write(&b"group("[..])?;

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
