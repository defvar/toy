use crate::config::{FileReadConfig, FileWriteConfig};
use crate::{FileReader, FileReaderBuilder, FileWriter, FileWriterBuilder, Row};
use failure::_core::fmt::Formatter;
use std::io;
use toy_core::channel::Outgoing;
use toy_core::data::{Frame, Map, Value};
use toy_core::error::ServiceError;
use toy_core::ServiceType;

pub struct FileReadContext {
    line: u32,
    reader: FileReader<Box<dyn io::Read + Send>>,
    buf: Row,
}

pub struct FileWriteContext {
    line: u32,
    writer: FileWriter<Box<dyn io::Write + Send>>,
}

impl std::fmt::Debug for FileWriteContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(std::any::type_name::<FileWriteContext>())
            .field("line", &self.line)
            .finish()
    }
}

impl std::fmt::Debug for FileReadContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(std::any::type_name::<FileReadContext>())
            .field("line", &self.line)
            .finish()
    }
}

pub fn new_read_context(
    _tp: ServiceType,
    config: FileReadConfig,
) -> Result<FileReadContext, ServiceError> {
    FileReaderBuilder::configure(&config)
        .map(|r| FileReadContext {
            line: 0u32,
            reader: r,
            buf: Row::new(),
        })
        .map_err(|e| e.into())
}

pub fn new_write_context(
    _tp: ServiceType,
    config: FileWriteConfig,
) -> Result<FileWriteContext, ServiceError> {
    FileWriterBuilder::configure(&config)
        .map(|w| FileWriteContext {
            line: 0u32,
            writer: w,
        })
        .map_err(|e| e.into())
}

pub async fn read(
    mut ctx: FileReadContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<FileReadContext, ServiceError> {
    while ctx.reader.read(&mut ctx.buf)? {
        let v = if ctx.reader.has_headers() {
            let v = ctx
                .reader
                .headers()?
                .iter()
                .zip(ctx.buf.iter())
                .map(|(h, v)| {
                    (
                        std::str::from_utf8(h).unwrap().to_string(),
                        Value::from(std::str::from_utf8(v).unwrap()),
                    )
                })
                .collect::<Map<_, _>>();
            Frame::from(v)
        } else {
            let v = ctx
                .buf
                .iter()
                .map(|c| Value::from(std::str::from_utf8(c).unwrap()))
                .collect::<Vec<_>>();
            Frame::from(v)
        };
        tx.send(Ok(v)).await?;
        ctx.line += 1;
    }
    Ok(ctx)
}

pub async fn write(
    mut ctx: FileWriteContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<FileWriteContext, ServiceError> {
    ctx.writer.write_value(req.value())?;
    ctx.line += 1;
    tx.send(Ok(Frame::none())).await?;
    Ok(ctx)
}
