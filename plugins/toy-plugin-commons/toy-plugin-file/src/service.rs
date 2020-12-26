use crate::config::{FileReadConfig, FileWriteConfig};
use crate::{FileReader, FileReaderBuilder, FileWriter, FileWriterBuilder};
use core::fmt::Formatter;
use std::io;
use toy_core::data::{Frame, Map, Value};
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::service::ServiceContext;
use toy_core::task::TaskContext;
use toy_core::ServiceType;
use toy_text_parser::Line;

pub struct FileReadContext {
    line: u32,
    reader: FileReader<Box<dyn io::Read + Send>>,
    buf: Line,
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
            buf: Line::new(),
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
    _task_ctx: TaskContext,
    mut ctx: FileReadContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<FileReadContext>, ServiceError> {
    while ctx.reader.read(&mut ctx.buf)? {
        let v = if ctx.reader.has_headers() {
            let v = ctx
                .reader
                .headers()?
                .iter()
                .zip(ctx.buf.iter())
                .map(|(h, v)| (String::from_utf8_lossy(h).to_string(), Value::from(v)))
                .collect::<Map<_, _>>();
            Frame::from(v)
        } else {
            let v = ctx.buf.iter().map(|c| Value::from(c)).collect::<Vec<_>>();
            Frame::from(v)
        };
        tx.send(Ok(v)).await?;
        ctx.line += 1;
    }
    Ok(ServiceContext::Complete(ctx))
}

pub async fn write(
    _task_ctx: TaskContext,
    mut ctx: FileWriteContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<FileWriteContext>, ServiceError> {
    match req.value() {
        Some(v) => {
            ctx.writer.write_value(v)?;
            ctx.line += 1;
            tx.send(Ok(Frame::none())).await?;
        }
        None => (),
    }

    Ok(ServiceContext::Ready(ctx))
}
