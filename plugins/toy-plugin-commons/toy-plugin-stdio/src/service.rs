use crate::config::{StdinConfig, StdoutConfig};
use std::future::Future;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use toy_core::prelude::*;

#[allow(dead_code)]
pub struct StdinContext {
    config: StdinConfig,
    reader: ReaderStream<tokio::io::Stdin>,
}

#[allow(dead_code)]
pub struct StdoutContext {
    config: StdoutConfig,
    writer: tokio::io::Stdout,
}

#[derive(Debug, Clone)]
pub struct Stdin;

impl Service for Stdin {
    type Context = StdinContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<StdinContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<StdinContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<StdinContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::source()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        mut ctx: Self::Context,
        _req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            let v = ctx.reader.next().await;
            match v {
                Some(Ok(bytes)) => {
                    tx.send_ok(Frame::from(&bytes[..])).await?;
                }
                Some(Err(e)) => {
                    return Err(ServiceError::error(e));
                }
                None => {}
            }
            Ok(ServiceContext::Ready(ctx))
        }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Stdin {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Stdin;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = StdinContext;
    type Config = StdinConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Stdin) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            let reader = ReaderStream::new(tokio::io::stdin());
            Ok(StdinContext { config, reader })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stdout;

impl Service for Stdout {
    type Context = StdoutContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<StdoutContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<StdoutContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<StdoutContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        mut ctx: Self::Context,
        req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            match req.value() {
                Some(v) => match v.parse_str() {
                    Some(str) => {
                        ctx.writer.write_all(str.as_bytes()).await?;
                        ctx.writer.write(&[b'\r', b'\n']).await?;
                    }
                    None => (),
                },
                None => (),
            };
            tx.send(Frame::none()).await?;
            Ok(ServiceContext::Ready(ctx))
        }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Stdout {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Stdout;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = StdoutContext;
    type Config = StdoutConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Stdout) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            let writer = tokio::io::stdout();
            Ok(StdoutContext { config, writer })
        }
    }
}
