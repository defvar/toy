use super::context::Context;
use crate::context::ContextFactory;
use std::any::Any;

pub type BoxContext = Box<dyn Context>;
pub type BoxContextFactory = Box<dyn ContextFactory<Context = BoxContext>>;

pub fn boxed<T>(factory: T) -> BoxContextFactory
where
    T: ContextFactory + 'static,
{
    Box::new(FactoryWrapper(factory))
}

struct FactoryWrapper<T: ContextFactory>(T);

impl<T> ContextFactory for FactoryWrapper<T>
where
    T: ContextFactory,
    T::Context: Context + 'static,
{
    type Context = BoxContext;

    fn new_context(&self) -> Self::Context {
        let item = self.0.new_context();
        ContextWrapper::boxed(item)
    }
}

impl<T> Context for Box<T>
where
    T: Context + ?Sized,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_mut()
    }
}

struct ContextWrapper<T>(T)
where
    T: Context + 'static;

impl<T> ContextWrapper<T>
where
    T: Context + 'static,
{
    fn boxed(context: T) -> BoxContext {
        Box::new(ContextWrapper(context))
    }
}

impl<T> Context for ContextWrapper<T>
where
    T: Context,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.0.as_any_mut()
    }
}
