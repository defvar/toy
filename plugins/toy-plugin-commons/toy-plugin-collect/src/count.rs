use crate::config::CountConfig;
use std::future::Future;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, ServiceType,
    TaskContext,
};

#[derive(Clone, Debug)]
pub struct Count;

pub struct CountContext {
    count: u64,
}

impl Service for Count {
    type Context = CountContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<CountContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<CountContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<CountContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        mut ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move {
            ctx.count += 1;
            Ok(ServiceContext::Ready(ctx))
        }
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
        task_ctx: TaskContext,
        ctx: Self::Context,
        mut tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        async move {
            let span = task_ctx.span();
            tracing::debug!(parent: span, send =?ctx.count);
            tx.send_ok(Frame::from(ctx.count)).await?;
            Ok(ServiceContext::Complete(ctx))
        }
    }
}

impl ServiceFactory for Count {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Count;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = CountContext;
    type Config = CountConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Count) }
    }

    fn new_context(&self, _tp: ServiceType, _config: Self::Config) -> Self::CtxFuture {
        async move { Ok(CountContext { count: 0 }) }
    }
}
