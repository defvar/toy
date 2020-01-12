use super::data::Frame;
use super::error::MessagingError;
use futures::channel::mpsc::Sender;
use std::any::Any;

pub trait Context {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ContextFactory {
    type Context: Context;
    fn new_context(&self) -> Self::Context;
}

#[derive(Clone)]
pub struct ServiceContext {
    sink: Sender<Result<Frame, MessagingError>>,
}

impl ServiceContext {
    pub fn new(sink: Sender<Result<Frame, MessagingError>>) -> ServiceContext {
        ServiceContext { sink }
    }

    pub fn send(&mut self, res: Result<Frame, MessagingError>) -> Result<(), MessagingError> {
        self.sink.try_send(res).map_err(|e| e.into())
    }
}

impl Context for ServiceContext {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
