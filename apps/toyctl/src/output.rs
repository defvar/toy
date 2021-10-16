use crate::error::Error;
use serde::Serialize;
use std::io::Write;

pub trait Output<W> {
    fn write(self, writer: W, pretty: bool) -> Result<(), Error>;
}

impl<T, W> Output<W> for T
where
    T: Serialize,
    W: Write,
{
    fn write(self, writer: W, pretty: bool) -> Result<(), Error> {
        JsonFormatter { data: self, pretty }.format(writer)
    }
}

pub trait OutputFormatter<W> {
    fn format(&self, writer: W) -> Result<(), Error>;
}

pub struct JsonFormatter<T> {
    data: T,
    pretty: bool,
}

impl<T, W> OutputFormatter<W> for JsonFormatter<T>
where
    T: Serialize,
    W: Write,
{
    fn format(&self, writer: W) -> Result<(), Error> {
        if self.pretty {
            toy_pack_json::pack_to_writer_pretty(writer, &self.data)?;
        } else {
            toy_pack_json::pack_to_writer(writer, &self.data)?;
        }
        Ok(())
    }
}
