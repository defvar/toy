use crate::config::LastConfig;
use std::future::Future;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceType, TaskContext,
};
use toy_core::service::ServiceFactory;

#[derive(Clone, Debug)]
pub struct Last;

pub struct LastContext {
    config: LastConfig,
    v: Option<Frame>,
}

impl Service for Last {
    type Context = LastContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<LastContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<LastContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<LastContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::sink()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move {
            Ok(ServiceContext::Ready(LastContext {
                config: ctx.config,
                v: Some(req),
            }))
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
            let _ = match ctx.v.clone() {
                Some(v) => {
                    let span = task_ctx.span();
                    tracing::debug!(parent: span, send =?v);
                    tx.send_ok(v).await
                }
                None => Ok(()),
            };
            Ok(ServiceContext::Complete(ctx))
        }
    }
}

impl ServiceFactory for Last {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Last;
    type Context = LastContext;
    type Config = LastConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Last) }
    }

    fn new_context(
        &self,
        _tp: ServiceType,
        config: Self::Config,
    ) -> Result<Self::Context, Self::InitError> {
        Ok(LastContext { config, v: None })
    }
}
