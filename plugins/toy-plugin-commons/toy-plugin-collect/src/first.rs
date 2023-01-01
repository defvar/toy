use crate::config::FirstConfig;
use std::future::Future;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, ServiceType,
    TaskContext,
};

#[derive(Clone, Debug)]
pub struct First;

pub struct FirstContext {}

impl Service for First {
    type Context = FirstContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<FirstContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<FirstContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<FirstContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            tracing::debug!(send =?req);
            tx.send_ok(req).await?;
            Ok(ServiceContext::Complete(ctx))
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

impl ServiceFactory for First {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = First;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = FirstContext;
    type Config = FirstConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(First) }
    }

    fn new_context(&self, _tp: ServiceType, _config: Self::Config) -> Self::CtxFuture {
        async move { Ok(FirstContext {}) }
    }
}
