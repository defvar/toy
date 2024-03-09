use crate::codec;
use toy_core::data::{Frame, Map, Value};
use v8::{Array, HandleScope, Local, Object};

pub fn array_length_property_name<'a>(scope: &mut HandleScope<'a>) -> v8::Local<'a, v8::String> {
    v8::String::new(scope, "length").unwrap()
}

pub fn get_length_object<'a>(scope: &mut HandleScope<'a>, obj: &Local<Object>) -> u32 {
    let js_key = array_length_property_name(scope);
    let length = obj.get(scope, js_key.into()).unwrap();
    length.uint32_value(scope).unwrap()
}

pub fn get_length_array<'a>(scope: &mut HandleScope<'a>, obj: &Local<Array>) -> u32 {
    let js_key = array_length_property_name(scope);
    let length = obj.get(scope, js_key.into()).unwrap();
    length.uint32_value(scope).unwrap()
}

pub fn encode<'a>(f: &Frame, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
    fn encode0<'a>(v: &Value, scope: &mut HandleScope<'a>) -> Local<'a, v8::Value> {
        match v {
            Value::Bool(_) => v8::Boolean::new(scope, v.as_bool().unwrap()).into(),
            Value::Integer(_) => match v.parse_integer::<i32>() {
                Some(i) => v8::Integer::new(scope, i).into(),
                None => v8::undefined(scope).into(),
            },
            Value::Number(_) => v8::Number::new(scope, v.parse_f64().unwrap()).into(),
            Value::String(v) => v8::String::new(scope, v).unwrap().into(),
            Value::Seq(ref vec) => {
                let r = v8::Array::new(scope, vec.len() as i32);
                let mut idx = 0;
                for v in vec {
                    let js_value = encode0(v, scope);
                    r.set_index(scope, idx, js_value);
                    idx += 1;
                }
                r.into()
            }
            Value::Map(ref map) => {
                let obj = v8::Object::new(scope);
                for (k, v) in map {
                    let js_key = v8::String::new(scope, k).unwrap().into();
                    let js_value = encode0(v, scope);
                    obj.set(scope, js_key, js_value);
                }
                obj.into()
            }
            _ => v8::undefined(scope).into(),
        }
    }
    let header = {
        let obj = v8::Object::new(scope);
        let js_key = v8::String::new(scope, "port").unwrap().into();
        let js_value = v8::Integer::new(scope, f.port() as i32).into();
        obj.set(scope, js_key, js_value);
        obj
    };
    let r = v8::Object::new(scope);
    let js_key = v8::String::new(scope, "header").unwrap().into();
    r.set(scope, js_key, header.into());
    if f.value().is_some() {
        let js_key = v8::String::new(scope, "payload").unwrap().into();
        let js_value = encode0(f.value().unwrap(), scope);
        r.set(scope, js_key, js_value);
    }

    r
}

pub fn decode(v: v8::Local<v8::Value>, scope: &mut v8::HandleScope) -> Value {
    fn decode0(v: &v8::Value, scope: &mut v8::HandleScope) -> Value {
        match v {
            v if v.is_null_or_undefined() => Value::None,
            v if v.is_boolean() => Value::from(v.boolean_value(scope)),
            v if v.is_int32() => Value::from(v.int32_value(scope)),
            v if v.is_uint32() => Value::from(v.uint32_value(scope)),
            v if v.is_big_int() => Value::from(v.integer_value(scope)),
            v if v.is_number() => Value::from(v.number_value(scope)),
            v if v.is_string() => Value::from(v.to_rust_string_lossy(scope)),
            vec if vec.is_array() => {
                let mut r = Vec::new();
                let vec = vec.to_object(scope).unwrap(); /* array */
                let length = codec::get_length_object(scope, &vec);
                for i in 0..length {
                    let v = vec.get_index(scope, i).unwrap();
                    r.push(decode0(&v, scope));
                }
                Value::from(r)
            }
            obj if obj.is_object() => {
                let mut r = Map::new();
                let obj = obj.to_object(scope).unwrap();
                let names = obj
                    .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
                    .unwrap();
                let length = codec::get_length_array(scope, &names);
                for i in 0..length {
                    let n = names.get_index(scope, i).unwrap();
                    let v = obj.get(scope, n.into()).unwrap();
                    let map_key = n.to_rust_string_lossy(scope);
                    let map_value = decode0(&v, scope);
                    r.insert(map_key, map_value);
                }
                Value::from(r)
            }
            _ => Value::None,
        }
    }

    let candidate = match v {
        obj if obj.is_object() => {
            let obj = v.to_object(scope).unwrap();
            let js_key = v8::String::new(scope, "payload").unwrap().into();
            obj.get(scope, js_key).unwrap()
        }
        _ => v,
    };

    decode0(&candidate, scope)
}
