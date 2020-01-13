use std::any::Any;

pub trait Context {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ContextFactory {
    type Context: Context;
    fn new_context(&self) -> Self::Context;
}
