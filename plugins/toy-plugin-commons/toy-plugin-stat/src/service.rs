use std::future::Future;
use toy_core::data::{Frame};
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::{Service, ServiceContext, ServiceFactory, TaskContext};
use toy_core::ServiceType;
use crate::config::*;
use crate::collector::{CpuCollector, MemoryCollector, StatCollector};

pub struct CpuContext {
    collector: CpuCollector,
}

pub struct MemoryContext {
    collector: MemoryCollector,
}

macro_rules! transform_service {
    ($service:ident, $config: ident, $ctx: ident) => {

        #[derive(Clone, Debug)]
        pub struct $service;

        impl Service for $service {
            type Context = $ctx;
            type Request = Frame;
            type Future = impl Future<Output=Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type UpstreamFinishFuture =
            impl Future<Output=Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type UpstreamFinishAllFuture =
            impl Future<Output=Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type Error = ServiceError;

            fn handle(&mut self, task_ctx: TaskContext, mut ctx: Self::Context, _req: Self::Request, mut tx: Outgoing<Self::Request>) -> Self::Future {
                async move {
                    let v = ctx.collector.to_stat_value();

                    tracing::debug!(parent: task_ctx.span(), "collect stat");

                    let f = Frame::from_value(v);
                    tx.send_ok(f).await?;
                    Ok(ServiceContext::Ready(ctx))
                }
            }

            fn upstream_finish(&mut self, _task_ctx: TaskContext, ctx: Self::Context, _req: Self::Request, _tx: Outgoing<Self::Request>) -> Self::UpstreamFinishFuture {
                async move { Ok(ServiceContext::Ready(ctx)) }
            }

            fn upstream_finish_all(&mut self, _task_ctx: TaskContext, ctx: Self::Context, _tx: Outgoing<Self::Request>) -> Self::UpstreamFinishAllFuture {
                async move { Ok(ServiceContext::Complete(ctx)) }
            }
        }

        impl ServiceFactory for $service {
            type Future = impl Future<Output=Result<Self::Service, Self::InitError>> + Send;
            type Service = $service;
            type CtxFuture = impl Future<Output=Result<Self::Context, Self::InitError>> + Send;
            type Context = $ctx;
            type Config = $config;
            type Request = Frame;
            type Error = ServiceError;
            type InitError = ServiceError;

            fn new_service(&self, _tp: ServiceType) -> Self::Future {
                async move { Ok($service) }
            }

            fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
                async move { Ok($ctx { collector: config.to_collector() }) }
            }
        }
    }
}

transform_service!(Cpu, CpuConfig, CpuContext);
transform_service!(Memory, MemoryConfig, MemoryContext);
