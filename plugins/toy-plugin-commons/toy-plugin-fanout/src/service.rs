use serde::Deserialize;
use std::future::Future;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, ServiceType,
};
use toy_core::task::TaskContext;
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Deserialize, Schema)]
pub struct BroadcastConfig {}

#[derive(Debug, Clone, Default)]
pub struct BroadcastContext {}

#[derive(Clone, Debug)]
pub struct Broadcast;

impl Service for Broadcast {
    type Context = BroadcastContext;
    type Request = Frame;
    type Future =
        impl Future<Output = Result<ServiceContext<BroadcastContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<BroadcastContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<BroadcastContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::fan_out_flow(20)
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        mut tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move {
            for p in tx.ports() {
                tx.send_ok_to(p, req.clone()).await?;
            }
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
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Broadcast {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Broadcast;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = BroadcastContext;
    type Config = BroadcastConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Broadcast) }
    }

    fn new_context(&self, _tp: ServiceType, _config: Self::Config) -> Self::CtxFuture {
        async move { Ok(BroadcastContext {}) }
    }
}
