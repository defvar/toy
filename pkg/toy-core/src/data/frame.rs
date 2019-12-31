use super::value::Value;
use super::Error;
use core::pin::Pin;
use core::task::{Context, Poll};
use failure::_core::marker::PhantomData;
use futures::Future;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame(Box<Value>);

impl Frame {
    pub fn from_map(fields: HashMap<String, Value>) -> Self {
        Frame(Box::new(Value::Map(fields)))
    }

    pub fn from_value(v: Value) -> Self {
        Frame(Box::new(v))
    }

    pub fn none() -> Frame {
        Frame::from_value(Value::None)
    }

    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self.0.borrow() {
            Value::Map(ref map) => map.get(key),
            _ => None,
        }
    }

    #[inline]
    pub fn get_idx(&self, idx: usize) -> Option<&Value> {
        match self.0.borrow() {
            Value::Seq(ref vec) => vec.get(idx),
            _ => None,
        }
    }

    #[inline]
    pub fn get_value(&self) -> &Value {
        self.0.borrow()
    }

    #[inline]
    pub fn future<E>(self) -> FrameFuture<E> {
        FrameFuture {
            inner: Some(self),
            _e: PhantomData,
        }
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

pub struct FrameFuture<E> {
    inner: Option<Frame>,
    _e: PhantomData<E>,
}

impl<E> Unpin for FrameFuture<E> {}

impl<E> Future for FrameFuture<E>
where
    E: Error,
{
    type Output = Result<Frame, E>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self
            .get_mut()
            .inner
            .take()
            .expect("A future should never be polled after it returns Ready")))
    }
}
