use std::future::Future;
use crate::config::TypedConfig;
use serde::{Deserialize, Serialize};
use toy_pack::Schema;
use toy_core::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub enum AllowedTypes {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    STR,
    TimeStamp,
}

impl Default for AllowedTypes {
    fn default() -> Self {
        AllowedTypes::STR
    }
}

#[derive(Clone, Debug)]
pub struct Typed;

pub struct TypedContext {
    config: TypedConfig,
}

impl Service for Typed {
    type Context = TypedContext;
    type Request = Frame;
    type Future = impl Future<Output=Result<ServiceContext<TypedContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
    impl Future<Output=Result<ServiceContext<TypedContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
    impl Future<Output=Result<ServiceContext<TypedContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        mut req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async move {
            match req.value_mut() {
                Some(v) => {
                    convert(v, &ctx.config);
                    tx.send_ok(req).await?;
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
        ctx: Self::Context,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Typed {
    type Future = impl Future<Output=Result<Self::Service, Self::InitError>> + Send;
    type Service = Typed;
    type CtxFuture = impl Future<Output=Result<Self::Context, Self::InitError>> + Send;
    type Context = TypedContext;
    type Config = TypedConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Typed) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            Ok(TypedContext { config })
        }
    }
}

pub fn convert(v: &mut Value, config: &TypedConfig) {
    match v {
        Value::Map(ref mut map) => {
            for (k, c) in &config.typed {
                if let Some(v) = map.get_mut(k) {
                    if let Some(new_v) = cast(&v, c.tp, c.default_value.as_ref().map(|x| x.as_str())) {
                        *v = new_v;
                    }
                }
            }
        }
        Value::Seq(_) => (),
        _ => ()
    }
}

pub(crate) fn cast(v: &Value, tp: AllowedTypes, default_value: Option<&str>) -> Option<Value> {
    if let Some(r) = match tp {
        AllowedTypes::U8 => v.parse_integer::<u8>().map(Value::from),
        AllowedTypes::U16 => v.parse_integer::<u16>().map(Value::from),
        AllowedTypes::U32 => v.parse_integer::<u32>().map(Value::from),
        AllowedTypes::U64 => v.parse_integer::<u64>().map(Value::from),
        AllowedTypes::I8 => v.parse_integer::<i8>().map(Value::from),
        AllowedTypes::I16 => v.parse_integer::<i16>().map(Value::from),
        AllowedTypes::I32 => v.parse_integer::<i32>().map(Value::from),
        AllowedTypes::I64 => v.parse_integer::<i64>().map(Value::from),
        AllowedTypes::F32 => v.parse_f32().map(Value::from),
        AllowedTypes::F64 => v.parse_f64().map(Value::from),
        AllowedTypes::STR => v.parse_str().map(Value::from),
        AllowedTypes::TimeStamp => v.parse_timestamp().map(Value::from)
    } {
        Some(r)
    } else if let Some(dv_str) = default_value {
        if let Some(dv) = cast(&Value::from(dv_str), tp, None) {
            Some(dv)
        } else {
            None
        }
    } else {
        None
    }
}
