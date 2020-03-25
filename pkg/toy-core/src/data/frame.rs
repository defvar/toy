use super::value::Value;
use crate::data::map::Map;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Frame {
    payload: Box<Value>,
}

impl Frame {
    pub fn from_value(v: Value) -> Self {
        Frame {
            payload: Box::new(v),
        }
    }

    #[inline]
    pub fn none() -> Frame {
        Frame::from_value(Value::None)
    }

    #[inline]
    pub fn value(&self) -> &Value {
        self.payload.borrow()
    }
}

macro_rules! impl_from_to_frame {
    ($t:ident) => {
        impl From<$t> for Frame {
            fn from(v: $t) -> Self {
                Frame::from_value(Value::from(v))
            }
        }
    };
}

impl_from_to_frame!(bool);
impl_from_to_frame!(u8);
impl_from_to_frame!(u16);
impl_from_to_frame!(u32);
impl_from_to_frame!(u64);
impl_from_to_frame!(i8);
impl_from_to_frame!(i16);
impl_from_to_frame!(i32);
impl_from_to_frame!(i64);
impl_from_to_frame!(f32);
impl_from_to_frame!(f64);
impl_from_to_frame!(String);
impl_from_to_frame!(char);

impl From<&String> for Frame {
    fn from(v: &String) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl From<Map<String, Value>> for Frame {
    fn from(v: Map<String, Value>) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl From<Vec<Value>> for Frame {
    fn from(v: Vec<Value>) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::none()
    }
}
