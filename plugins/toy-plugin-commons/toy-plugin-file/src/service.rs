use crate::config::{ReadConfig, WriteConfig};
use crate::file_reader::FileReader;
use crate::file_reader_builder::FileReaderBuilder;
use crate::file_writer::FileWriter;
use crate::file_writer_builder::FileWriterBuilder;
use core::fmt::Formatter;
use std::future::Future;
use std::io;
use toy_core::prelude::*;
use toy_text_parser::Line;

pub struct ReadContext {
    line: u32,
    reader: FileReader,
    buf: Line,
}

pub struct WriteContext {
    line: u32,
    writer: FileWriter<Box<dyn io::Write + Send>>,
}

impl std::fmt::Debug for WriteContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(std::any::type_name::<WriteContext>())
            .field("line", &self.line)
            .finish()
    }
}

impl std::fmt::Debug for ReadContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(std::any::type_name::<ReadContext>())
            .field("line", &self.line)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Read;

impl Service for Read {
    type Context = ReadContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<ReadContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<ReadContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<ReadContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::source()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move { read(task_ctx, ctx, req, tx).await }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Read {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Read;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = ReadContext;
    type Config = ReadConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Read) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            FileReaderBuilder::configure(&config)
                .map(|r| ReadContext {
                    line: 0u32,
                    reader: r,
                    buf: Line::new(),
                })
                .map_err(|e| e.into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Write;

impl Service for Write {
    type Context = WriteContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<WriteContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<WriteContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<WriteContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move { write(task_ctx, ctx, req, tx).await }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Write {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Write;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = WriteContext;
    type Config = WriteConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Write) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            FileWriterBuilder::configure(&config)
                .map(|w| WriteContext {
                    line: 0u32,
                    writer: w,
                })
                .map_err(|e| e.into())
        }
    }
}

async fn read(
    _task_ctx: TaskContext,
    mut ctx: ReadContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<ReadContext>, ServiceError> {
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

async fn write(
    _task_ctx: TaskContext,
    mut ctx: WriteContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<WriteContext>, ServiceError> {
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
