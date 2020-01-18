use super::value::Value;
use std::borrow::Borrow;
use std::collections::HashMap;

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
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self.payload.borrow() {
            Value::Map(ref map) => map.get(key),
            _ => None,
        }
    }

    #[inline]
    pub fn get_idx(&self, idx: usize) -> Option<&Value> {
        match self.payload.borrow() {
            Value::Seq(ref vec) => vec.get(idx),
            _ => None,
        }
    }

    #[inline]
    pub fn get_value(&self) -> &Value {
        self.payload.borrow()
    }
}

impl From<&String> for Frame {
    fn from(v: &String) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl From<String> for Frame {
    fn from(v: String) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl From<u32> for Frame {
    fn from(v: u32) -> Self {
        Frame::from_value(Value::from(v))
    }
}

impl From<HashMap<String, Value>> for Frame {
    fn from(v: HashMap<String, Value>) -> Self {
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
