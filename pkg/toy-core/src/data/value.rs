use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),
    Bytes(Vec<u8>),

    None,
    Some(Box<Value>),

    Seq(Vec<Value>),
    Map(HashMap<String, Value>),

    Unit,
}

macro_rules! impl_from_to_value {
    ($t:ident, $variant: ident) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Self {
                Value::$variant(v)
            }
        }
    };
}

impl_from_to_value!(bool, Bool);
impl_from_to_value!(u8, U8);
impl_from_to_value!(u16, U16);
impl_from_to_value!(u32, U32);
impl_from_to_value!(u64, U64);
impl_from_to_value!(i8, I8);
impl_from_to_value!(i16, I16);
impl_from_to_value!(i32, I32);
impl_from_to_value!(i64, I64);
impl_from_to_value!(f32, F32);
impl_from_to_value!(f64, F64);
impl_from_to_value!(String, String);
impl_from_to_value!(char, Char);

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::String(v.to_string())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(v: HashMap<String, Value>) -> Self {
        Value::Map(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Seq(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(r) => Value::Some(Box::new(r.into())),
            None => Value::None,
        }
    }
}
