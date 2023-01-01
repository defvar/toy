use crate::error::LuaFunctionError;
use serde::{Deserialize, Serialize};
use std::future::Future;
use toy_core::prelude::*;
use toy_pack::Schema;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct LuaFunctionConfig {
    pub code: String,
}

pub struct LuaFunctionContext {
    config: LuaFunctionConfig,
    raw: rlua::Lua,
}

#[derive(Debug, Clone)]
pub struct LuaFunction;

impl Service for LuaFunction {
    type Context = LuaFunctionContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<Self::Context>, Self::Error>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<Self::Context>, Self::Error>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<Self::Context>, Self::Error>> + Send;
    type Error = ServiceError;

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        mut req: Self::Request,
        mut tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        let code = ctx.config.code.clone();
        async move {
            let req_lua = req.clone();
            let new_value = ctx
                .raw
                .context(|lua_ctx| {
                    encode_and_set(&lua_ctx, req_lua)?;
                    lua_ctx.load(&code).exec()?;
                    let v = get_and_decode(&lua_ctx).unwrap();
                    Result::<Value, LuaFunctionError>::Ok(v)
                })
                .map_err(|e| ServiceError::error(e))?;
            match req.value_mut() {
                Some(v) => {
                    *v = new_value;
                    tx.send_ok(req).await?;
                }
                None => {}
            };
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

impl ServiceFactory for LuaFunction {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = LuaFunction;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = LuaFunctionContext;
    type Config = LuaFunctionConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(LuaFunction) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move {
            let raw = rlua::Lua::new();
            Ok(LuaFunctionContext { config, raw })
        }
    }
}

fn encode_and_set(lua_ctx: &rlua::Context, f: Frame) -> Result<(), LuaFunctionError> {
    fn encode0<'b>(lua_ctx: &rlua::Context<'b>, v: Value) -> rlua::Result<rlua::Value<'b>> {
        let v = match v {
            Value::Bool(v) => rlua::Value::Boolean(v),
            Value::Integer(v) => rlua::Value::Integer(v),
            Value::Number(v) => rlua::Value::Number(v),
            Value::String(v) => rlua::Value::String(lua_ctx.create_string(&v)?),
            Value::Bytes(_) => rlua::Value::Nil,
            Value::None => rlua::Value::Nil,
            Value::Seq(v) => {
                let table = lua_ctx.create_table()?;
                for (idx, element) in v.into_iter().enumerate() {
                    let rv = encode0(lua_ctx, element)?;
                    table.set(idx, rv)?;
                }
                rlua::Value::Table(table)
            }
            Value::Map(v) => {
                let table = lua_ctx.create_table()?;
                for (k, v) in v.into_iter() {
                    let rv = encode0(lua_ctx, v)?;
                    table.set(k, rv)?;
                }
                rlua::Value::Table(table)
            }
            Value::TimeStamp(_) => rlua::Value::Nil,
        };
        Ok(v)
    }

    let toy = lua_ctx.create_table()?;
    let header = lua_ctx.create_table()?;
    header.set("port", f.port())?;
    let mut payload = rlua::Value::Nil;
    if f.value().is_some() {
        payload = encode0(lua_ctx, f.into_value().unwrap())?;
    }
    toy.set("header", header)?;
    toy.set("payload", payload)?;
    lua_ctx.globals().set("toy", toy)?;
    Ok(())
}

fn get_and_decode(lua_ctx: &rlua::Context) -> Result<Value, LuaFunctionError> {
    fn decode0(lua_value: rlua::Value) -> Result<Value, LuaFunctionError> {
        Ok(match lua_value {
            rlua::Value::Table(rv) => {
                let len = rv.len()?;
                if len > 0 {
                    let mut vec = Vec::with_capacity(len as usize);
                    for element in rv.sequence_values::<rlua::Value>() {
                        let element = element?;
                        let v = decode0(element)?;
                        vec.push(v);
                    }
                    Value::from(vec)
                } else {
                    let mut map = Map::new();
                    for pair in rv.pairs::<rlua::Value, rlua::Value>() {
                        let (k, v) = pair?;
                        let k = decode0(k)?;
                        let v = decode0(v)?;
                        map.insert(k.parse_str().unwrap().to_owned(), v);
                    }
                    Value::from(map)
                }
            }
            rlua::Value::String(rv) => Value::from(rv.to_str()?),
            rlua::Value::Integer(rv) => Value::from(rv),
            rlua::Value::Number(rv) => Value::from(rv),
            rlua::Value::Boolean(rv) => Value::from(rv),
            rlua::Value::Nil => Value::None,
            _ => Value::None,
        })
    }
    let lua_value = lua_ctx.globals().get::<_, rlua::Value>("toy")?;
    let candidate = match &lua_value {
        rlua::Value::Table(rv) => {
            if let Ok(true) = rv.contains_key("payload") {
                rv.get("payload")?
            } else {
                lua_value
            }
        }
        _ => lua_value,
    };
    let v = decode0(candidate)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::get_and_decode;

    #[test]
    fn decode_table_map() {
        let raw = rlua::Lua::new();
        let v = raw
            .context(|ctx| {
                let table = ctx.create_table()?;
                table.set("number", 1)?;
                table.set("message", "hello")?;
                ctx.globals().set("toy", table)?;

                let v = get_and_decode(&ctx).unwrap();
                Result::<_, rlua::Error>::Ok(v)
            })
            .unwrap();
        assert_eq!(v.path("message").unwrap(), "hello");
        assert_eq!(v.path("number").unwrap(), 1i64);
    }

    #[test]
    fn decode_table_map_nested() {
        let raw = rlua::Lua::new();
        let v = raw
            .context(|ctx| {
                let table = ctx.create_table()?;
                let inner = ctx.create_table()?;
                inner.set("number", 1)?;
                table.set("message", "hello")?;
                table.set("inner", inner)?;
                ctx.globals().set("toy", table)?;

                let v = get_and_decode(&ctx).unwrap();
                Result::<_, rlua::Error>::Ok(v)
            })
            .unwrap();
        assert_eq!(v.path("message").unwrap(), "hello");
        assert_eq!(v.path("inner.number").unwrap(), 1i64);
    }

    #[test]
    fn decode_table_seq() {
        let raw = rlua::Lua::new();
        let v = raw
            .context(|ctx| {
                let table = ctx.create_table()?;
                table.set(1, "a")?;
                table.set(2, "b")?;
                ctx.globals().set("toy", table)?;

                let v = get_and_decode(&ctx).unwrap();
                Result::<_, rlua::Error>::Ok(v)
            })
            .unwrap();

        assert_eq!(v.path("0").unwrap(), "a");
        assert_eq!(v.path("1").unwrap(), "b");
        assert_eq!(v.path("2"), None);
    }
}
