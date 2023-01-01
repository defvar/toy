use serde::{Deserialize, Serialize};
use std::future::Future;
use tokio::time::Duration;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, ServiceType,
    TaskContext,
};
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
pub struct TickConfig {
    interval_millis: u64,
    start: u64,
    end: Option<u64>,
}

pub struct TickContext {
    count: u64,
    config: TickConfig,
}

#[derive(Debug, Clone)]
pub struct Tick;

impl Service for Tick {
    type Context = TickContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<TickContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<TickContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<TickContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::source()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        mut ctx: Self::Context,
        _req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            tokio::time::sleep(Duration::from_millis(ctx.config.interval_millis)).await;
            let span = task_ctx.span();
            tracing::debug!(parent: span, send = ctx.count);

            tx.send_ok(Frame::from(ctx.count)).await?;
            match ctx.config.end {
                Some(end) if end <= ctx.count => {
                    tracing::debug!(parent: span, "count end");
                    Ok(ServiceContext::Complete(ctx))
                }
                _ => {
                    ctx.count += 1;
                    Ok(ServiceContext::Next(ctx))
                }
            }
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

impl ServiceFactory for Tick {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Tick;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = TickContext;
    type Config = TickConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Tick) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            Ok(TickContext {
                count: config.start,
                config,
            })
        }
    }
}
