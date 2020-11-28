use super::value::Value;
use crate::data::map::Map;
use crate::mpsc::OutgoingMessage;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone)]
pub struct Frame {
    header: Header,
    payload: Box<Value>,
}

#[derive(Debug, Clone)]
struct Header {
    port: u8,
    frame_type: FrameType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameType {
    Data,
    Signal(Signal),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signal {
    Stop,
}

impl Frame {
    pub fn from_value(v: Value) -> Self {
        Frame {
            header: Header::data(),
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

    #[inline]
    pub fn value_mut(&mut self) -> &mut Value {
        self.payload.borrow_mut()
    }

    #[inline]
    pub fn port(&self) -> u8 {
        self.header.port
    }

    #[inline]
    fn set_port(&mut self, port: u8) {
        self.header.port = port;
    }

    pub fn stop() -> Frame {
        Frame {
            header: Header::signal(Signal::Stop),
            payload: Box::new(Value::None),
        }
    }

    pub fn is_stop(&self) -> bool {
        self.header.frame_type == FrameType::Signal(Signal::Stop)
    }
}

impl Header {
    pub fn data() -> Self {
        Self {
            port: 0,
            frame_type: FrameType::Data,
        }
    }

    pub fn signal(v: Signal) -> Self {
        Self {
            port: 0,
            frame_type: FrameType::Signal(v),
        }
    }
}

impl OutgoingMessage for Frame {
    fn set_port(&mut self, port: u8) {
        self.set_port(port)
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
