use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

pub struct Pivot<'a> {
    row_key: Vec<&'a str>,
    column_key: Vec<&'a str>,
    value_column: &'a str,
}

impl<'a> Pivot<'a> {
    pub fn with(
        row_key: Vec<&'a str>,
        column_key: Vec<&'a str>,
        value_column: &'a str,
    ) -> Pivot<'a> {
        Pivot {
            row_key,
            column_key,
            value_column,
        }
    }
}

impl<'a> FluxPart for Pivot<'a> {
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut r = 0;
        r += writer.write(&b"pivot(rowKey: ["[..])?;
        r += self
            .row_key
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

        r += writer.write(&b"], columnKey: ["[..])?;
        r += self
            .column_key
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

        r += writer.write(&b"], valueColumn: "[..])?;
        r += writer.write(&[b'\"'])?;
        r += writer.write(self.value_column.as_bytes())?;
        r += writer.write(&[b'\"'])?;
        r += writer.write(&[b')'])?;

        Ok(r)
    }
}
