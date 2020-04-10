use crate::config::TypedConfig;
use toy_core::data::Value;

// rust types
const TYPE_NAME_U8: &'static str = "u8";
const TYPE_NAME_U16: &'static str = "u16";
const TYPE_NAME_U32: &'static str = "u32";
const TYPE_NAME_U64: &'static str = "u64";
const TYPE_NAME_I8: &'static str = "i8";
const TYPE_NAME_I16: &'static str = "i16";
const TYPE_NAME_I32: &'static str = "i32";
const TYPE_NAME_I64: &'static str = "i64";
const TYPE_NAME_F32: &'static str = "f32";
const TYPE_NAME_F64: &'static str = "f64";
const TYPE_NAME_STR: &'static str = "str";
const TYPE_NAME_STRING: &'static str = "string";

const TYPE_NAMES: &'static [&'static str] = &[
    TYPE_NAME_U8,
    TYPE_NAME_U16,
    TYPE_NAME_U32,
    TYPE_NAME_U64,
    TYPE_NAME_I8,
    TYPE_NAME_I16,
    TYPE_NAME_I32,
    TYPE_NAME_I64,
    TYPE_NAME_F32,
    TYPE_NAME_F64,
    TYPE_NAME_STR,
    TYPE_NAME_STRING,
];

pub fn is_valid_type_name(tp: &str) -> bool {
    TYPE_NAMES.contains(&tp)
}

pub fn convert(v: &mut Value, config: &TypedConfig) {
    match v {
        Value::Map(ref mut map) => {
            for (k, c) in &config.typed {
                if let Some(v) = map.get_mut(k) {
                    if let Some(new_v) = cast(&v, c.tp.as_str()) {
                        *v = new_v;
                    } else {
                        //default value
                        if let Some(ref dv_str) = c.default_value {
                            if let Some(dv) = cast(&Value::from(dv_str), c.tp.as_str()) {
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

pub(crate) fn cast(v: &Value, tp: &str) -> Option<Value> {
    match tp {
        TYPE_NAME_U8 => v.parse_integer::<u8>(),
        TYPE_NAME_U16 => v.parse_integer::<u16>(),
        TYPE_NAME_U32 => v.parse_integer::<u32>(),
        TYPE_NAME_U64 => v.parse_integer::<u64>(),
        TYPE_NAME_I8 => v.parse_integer::<i8>(),
        TYPE_NAME_I16 => v.parse_integer::<i16>(),
        TYPE_NAME_I32 => v.parse_integer::<i32>(),
        TYPE_NAME_I64 => v.parse_integer::<i64>(),
        TYPE_NAME_F32 => v.parse_integer::<f32>(),
        TYPE_NAME_F64 => v.parse_integer::<f64>(),
        TYPE_NAME_STR | TYPE_NAME_STRING => v.parse_str(),
        _ => None,
    }
}
