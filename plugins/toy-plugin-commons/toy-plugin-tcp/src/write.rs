use serde::{Deserialize, Serialize};
use std::future::Future;
use tokio::io::{AsyncWriteExt, BufWriter};
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, TaskContext,
    Value,
};
use toy_core::ServiceType;
use toy_pack::Schema;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct TcpWriteConfig {
    addr: String,
}

#[allow(dead_code)]
pub struct TcpWriteContext {
    config: TcpWriteConfig,
    raw: BufWriter<tokio::net::TcpStream>,
}

#[derive(Debug, Clone)]
pub struct TcpWrite;

impl Service for TcpWrite {
    type Context = TcpWriteContext;
    type Request = Frame;
    type Future =
        impl Future<Output = Result<ServiceContext<TcpWriteContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<TcpWriteContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<TcpWriteContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        mut ctx: Self::Context,
        req: Self::Request,
        _tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            match req.value() {
                Some(v) => {
                    match v {
                        Value::Bytes(bytes) => ctx.raw.write_all(bytes).await?,
                        Value::String(s) => ctx.raw.write_all(s.as_bytes()).await?,
                        _ => {
                            return Err(ServiceError::error(
                                "must be type Value::Bytes or Value::String.",
                            ))
                        }
                    };
                }
                None => (),
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
        mut ctx: Self::Context,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move {
            ctx.raw.flush().await?;
            Ok(ServiceContext::Complete(ctx))
        }
    }
}

impl ServiceFactory for TcpWrite {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = TcpWrite;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = TcpWriteContext;
    type Config = TcpWriteConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(TcpWrite) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        let addr = config.addr.clone();
        async move {
            Ok(TcpWriteContext {
                config,
                raw: BufWriter::new(tokio::net::TcpStream::connect(addr).await?),
            })
        }
    }
}
