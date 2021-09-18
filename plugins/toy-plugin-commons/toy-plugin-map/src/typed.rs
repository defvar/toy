use crate::config::TypedConfig;
use serde::{Deserialize, Serialize};
use toy_core::data::Value;
use toy_pack::Schema;

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
}

impl Default for AllowedTypes {
    fn default() -> Self {
        AllowedTypes::STR
    }
}

pub fn convert(v: &mut Value, config: &TypedConfig) {
    match v {
        Value::Map(ref mut map) => {
            for (k, c) in &config.typed {
                if let Some(v) = map.get_mut(k) {
                    if let Some(new_v) = cast(&v, c.tp) {
                        *v = new_v;
                    } else {
                        //default value
                        if let Some(ref dv_str) = c.default_value {
                            if let Some(dv) = cast(&Value::from(dv_str), c.tp) {
                                *v = dv;
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }
}

pub(crate) fn cast(v: &Value, tp: AllowedTypes) -> Option<Value> {
    match tp {
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
    }
}
