use crate::config::FixedSizeConfig;
use std::future::Future;
use toy_core::data::{Frame, Value};
use toy_core::error::{OutgoingError, ServiceError};
use toy_core::mpsc::Outgoing;
use toy_core::prelude::{
    map_value, PortType, Service, ServiceContext, ServiceFactory, TaskContext,
};
use toy_core::ServiceType;

#[derive(Clone, Debug)]
pub struct FixedSize;

pub struct FixedSizeContext {
    config: FixedSizeConfig,
    buf: Option<Vec<Value>>,
    timestamps: Option<Vec<Value>>,
}

impl FixedSizeContext {
    pub fn len(&self) -> usize {
        self.buf.as_ref().unwrap().len()
    }
}

impl Service for FixedSize {
    type Context = FixedSizeContext;
    type Request = Frame;
    type Future =
        impl Future<Output = Result<ServiceContext<FixedSizeContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<FixedSizeContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<FixedSizeContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::flow()
    }

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        mut ctx: Self::Context,
        req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            let v = req.into_value().unwrap_or(Value::None);
            ctx.buf.as_mut().map(|x| x.push(v));
            ctx.timestamps.as_mut().map(|x| x.push(Value::now()));
            if ctx.len() >= ctx.config.size {
                flush(&mut ctx, &mut tx).await?;
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
        mut tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move {
            flush(&mut ctx, &mut tx).await?;
            Ok(ServiceContext::Complete(ctx))
        }
    }
}

async fn flush(ctx: &mut FixedSizeContext, tx: &mut Outgoing<Frame>) -> Result<(), OutgoingError> {
    if ctx.len() > 0 {
        let v = map_value! {
            "payload" => ctx.buf.take(),
            "timestamps" => ctx.timestamps.take()
        };
        ctx.buf = Some(Vec::with_capacity(ctx.config.size));
        ctx.timestamps = Some(Vec::with_capacity(ctx.config.size));
        tx.send_ok(Frame::from_value(v)).await
    } else {
        Ok(())
    }
}

impl ServiceFactory for FixedSize {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = FixedSize;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = FixedSizeContext;
    type Config = FixedSizeConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(FixedSize) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        let s = config.size;
        async move {
            Ok(FixedSizeContext {
                config,
                buf: Some(Vec::with_capacity(s)),
                timestamps: Some(Vec::with_capacity(s)),
            })
        }
    }
}
