use super::value::Value;
use crate::data::map::Map;
use crate::mpsc::OutgoingMessage;
use std::fmt;

/// A Value with header information added, which is used when transferring between channels.
#[derive(Debug, Clone)]
pub struct Frame {
    header: Header,
    payload: Option<Value>,
}

#[derive(Debug, Clone)]
struct Header {
    port: u8,
    frame_type: FrameType,
}

/// Represents the type of frame.
///
/// In the case of Data, the payload exists, but in the case of Signal, it does not exist.
/// Generally, the message exchanged between nodes is Data.
/// Signal is used for special messages for each node.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameType {
    Data,
    Signal(Signal),
}

/// Special messages for each node.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signal {
    Stop,
    UpstreamFinish,
}

impl Frame {
    pub fn from_value(v: Value) -> Self {
        Frame {
            header: Header::data(),
            payload: Some(v),
        }
    }

    #[inline]
    pub fn none() -> Frame {
        Frame::from_value(Value::None)
    }

    #[inline]
    pub fn value(&self) -> Option<&Value> {
        match &self.payload {
            Some(v) => Some(v),
            None => None,
        }
    }

    #[inline]
    pub fn value_mut(&mut self) -> Option<&mut Value> {
        self.payload.as_mut().map(|x| x)
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
            payload: None,
        }
    }

    pub fn upstream_finish() -> Frame {
        Frame {
            header: Header::signal(Signal::UpstreamFinish),
            payload: None,
        }
    }

    pub fn is_stop(&self) -> bool {
        self.header.frame_type == FrameType::Signal(Signal::Stop)
    }

    pub fn is_upstream_finish(&self) -> bool {
        self.header.frame_type == FrameType::Signal(Signal::UpstreamFinish)
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

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Frame { ")?;
        self.header.fmt(f)?;
        if self.payload.is_some() {
            f.write_str(", ")?;
            f.write_str("payload: ")?;
            self.payload.as_ref().unwrap().fmt(f)?;
        }
        f.write_str(" } ")?;
        Ok(())
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("frame_type", &self.frame_type)
            .field("port", &self.port)
            .finish()
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
