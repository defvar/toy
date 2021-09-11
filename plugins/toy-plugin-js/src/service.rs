use crate::config::FunctionConfig;
use quick_js::{Context, JsValue};
use std::cell::RefCell;
use std::collections::HashMap;
use toy_core::prelude::*;

#[derive(Debug, Clone)]
pub struct FunctionContext {
    config: FunctionConfig,
}

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = RefCell::new(None)
}

fn current() -> Option<Context> {
    CONTEXT.with(|ctx| ctx.borrow_mut().take())
}

fn set_if_empty(func: impl Fn() -> Result<Context, ServiceError>) -> Result<(), ServiceError> {
    CONTEXT.with(|ctx| {
        if ctx.borrow().is_none() {
            let r = func();
            match r {
                Ok(new_c) => {
                    ctx.borrow_mut().replace(new_c);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    })
}

fn replace(c: Context) {
    CONTEXT.with(|ctx| {
        ctx.borrow_mut().replace(c);
    })
}

pub fn new_function_context(
    _tp: ServiceType,
    config: FunctionConfig,
) -> Result<FunctionContext, ServiceError> {
    set_if_empty(|| Context::new().map_err(|e| ServiceError::error(e)))?;
    Ok(FunctionContext { config })
}

pub async fn js_function(
    _task_ctx: TaskContext,
    ctx: FunctionContext,
    mut req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<FunctionContext>, ServiceError> {
    let new_value = {
        let mut c = match current() {
            Some(c) => c,
            None => return Err(ServiceError::error("js context not initialized.")),
        };

        let frame_js = req.clone();
        c.add_callback("toy", move || encode(&frame_js))
            .map_err(|e| ServiceError::error(e))?;

        let js_ret = c
            .eval(&ctx.config.code)
            .map_err(|e| ServiceError::error(e))?;

        c = c.reset().map_err(|e| ServiceError::error(e))?;
        replace(c);

        decode(&js_ret)
    };
    match req.value_mut() {
        Some(v) => {
            *v = new_value;
            tx.send_ok(req).await?;
        }
        None => {}
    };

    Ok(ServiceContext::Ready(ctx))
}

fn encode(f: &Frame) -> JsValue {
    fn encode0(v: &Value) -> JsValue {
        match v {
            Value::Bool(v) => JsValue::from(*v),
            Value::U8(v) => JsValue::from(*v),
            Value::U16(v) => JsValue::from(*v),
            Value::U32(v) => JsValue::from(*v),
            Value::I8(v) => JsValue::from(*v),
            Value::I16(v) => JsValue::from(*v),
            Value::I32(v) => JsValue::from(*v),
            Value::U64(_) | Value::I64(_) => match v.parse_integer::<i32>() {
                Some(n) => JsValue::Int(n),
                None => JsValue::Undefined,
            },
            Value::F32(v) => JsValue::from(*v as f64),
            Value::F64(v) => JsValue::from(*v as f64),
            Value::String(v) => JsValue::String(v.clone()),
            Value::Seq(ref vec) => {
                let mut r = Vec::new();
                for v in vec {
                    r.push(encode0(v));
                }
                JsValue::from(r)
            }
            Value::Map(ref map) => {
                let mut r = HashMap::new();
                for (k, v) in map {
                    r.insert(k, encode0(v));
                }
                JsValue::from(r)
            }
            _ => JsValue::Undefined,
        }
    }
    let header = {
        let mut h = HashMap::new();
        h.insert("port", JsValue::from(f.port()));
        JsValue::from(h)
    };
    let mut r = HashMap::new();
    r.insert("header", header);
    if f.value().is_some() {
        r.insert("payload", encode0(f.value().unwrap()));
    }
    JsValue::from(r)
}

fn decode(v: &JsValue) -> Value {
    fn decode0(v: &JsValue) -> Value {
        match v {
            JsValue::Undefined => Value::None,
            JsValue::Null => Value::None,
            JsValue::Bool(v) => Value::from(*v),
            JsValue::Int(v) => Value::from(*v),
            JsValue::Float(v) => Value::from(*v),
            JsValue::String(v) => Value::from(v),
            JsValue::Array(ref vec) => {
                let mut r = Vec::new();
                for v in vec {
                    r.push(decode(v));
                }
                Value::from(r)
            }
            JsValue::Object(ref map) => {
                let mut r = Map::new();
                for (k, v) in map {
                    r.insert(k.clone(), decode(v));
                }
                Value::from(r)
            }
            _ => Value::None,
        }
    }

    let candidate = match v {
        JsValue::Object(ref map) => {
            if map.contains_key("payload") {
                map.get("payload").unwrap()
            } else {
                v
            }
        }
        _ => v,
    };
    decode0(candidate)
}

#[cfg(test)]
mod tests {
    use crate::service::{decode, encode};
    use quick_js::JsValue;
    use std::collections::HashMap;
    use toy_core::prelude::*;

    #[test]
    fn encode_object() {
        let value = map_value! {
            "message" => "from rust",
        };
        let frame = Frame::from_value(value);
        let js_value = encode(&frame);
        match js_value {
            JsValue::Object(ref map) => match map.get("payload").unwrap() {
                JsValue::Object(ref payload) => match payload.get("message").unwrap() {
                    JsValue::String(s) => assert_eq!(s, "from rust"),
                    _ => panic!("invalid type."),
                },
                _ => panic!("invalid type."),
            },
            _ => panic!("invalid type."),
        }
    }

    #[test]
    fn decode_object() {
        let v = {
            let mut m = HashMap::new();
            m.insert("message".to_string(), JsValue::from("from js"));
            decode(&JsValue::Object(m))
        };
        assert_eq!(v.path("message").unwrap(), "from js");
    }

    #[test]
    fn decode_seq() {
        let v = {
            let mut vec = Vec::new();
            vec.push(JsValue::from("from js 1"));
            vec.push(JsValue::from("from js 2"));
            decode(&JsValue::Array(vec))
        };
        assert_eq!(v.path("0").unwrap(), "from js 1");
        assert_eq!(v.path("1").unwrap(), "from js 2");
    }
}
