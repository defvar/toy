use std::future::Future;
use toy_core::data::{Frame, Value};
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::{Service, ServiceContext, ServiceFactory, TaskContext};
use toy_core::ServiceType;
use crate::config::FilterConfig;

#[derive(Clone, Debug)]
pub struct Filter;

pub struct FilterContext {
    config: FilterConfig,
}

impl Service for Filter {
    type Context = FilterContext;
    type Request = Frame;
    type Future = impl Future<Output=Result<ServiceContext<FilterContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
    impl Future<Output=Result<ServiceContext<FilterContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
    impl Future<Output=Result<ServiceContext<FilterContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn handle(&mut self, task_ctx: TaskContext, ctx: Self::Context, req: Self::Request, mut tx: Outgoing<Self::Request>) -> Self::Future {
        async move {
            let span = task_ctx.span();

            if req.value().is_some() {
                let all_match = ctx.config.preds().iter().all(|x| {
                    let candidate = match req.value().unwrap() {
                        v @ Value::Map(_) | v @ Value::Seq(_) => {
                            v.path(x.field())
                        }
                        other => Some(other),
                    };
                    let r = candidate.map(|v| x.is_match(v));
                    tracing::debug!(parent: span, predicate = %x, ?candidate, matched = r);
                    r.unwrap_or(false)
                });

                if all_match {
                    tx.send_ok(req).await?
                }
            }
            Ok(ServiceContext::Next(ctx))
        }
    }

    fn upstream_finish(&mut self, _task_ctx: TaskContext, ctx: Self::Context, _req: Self::Request, _tx: Outgoing<Self::Request>) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(&mut self, _task_ctx: TaskContext, ctx: Self::Context, _tx: Outgoing<Self::Request>) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Filter {
    type Future = impl Future<Output=Result<Self::Service, Self::InitError>> + Send;
    type Service = Filter;
    type CtxFuture = impl Future<Output=Result<Self::Context, Self::InitError>> + Send;
    type Context = FilterContext;
    type Config = FilterConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Filter) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move { Ok(FilterContext { config }) }
    }
}