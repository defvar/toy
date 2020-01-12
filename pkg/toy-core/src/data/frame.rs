use super::value::Value;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameState {
    Data,
    End,
}

#[derive(Debug, Clone)]
pub struct Frame {
    state: FrameState,
    payload: Box<Value>,
}

impl Frame {
    pub fn from_value(v: Value) -> Self {
        Frame {
            state: FrameState::Data,
            payload: Box::new(v),
        }
    }

    pub fn from_value_and_state<T: Into<Value>>(v: T, state: FrameState) -> Self {
        Frame {
            state,
            payload: Box::new(v.into()),
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

    #[inline]
    pub fn to_end_frame(&self) -> Frame {
        let mut r = self.clone();
        r.state = FrameState::End;
        r
    }

    #[inline]
    pub fn into_end_frame(mut self) -> Frame {
        self.state = FrameState::End;
        self
    }

    pub fn is_end_frame(&self) -> bool {
        self.state == FrameState::End
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

impl From<HashMap<String, Value>> for Frame {
    fn from(v: HashMap<String, Value>) -> Self {
        Frame::from_value(Value::from(v))
    }
}
