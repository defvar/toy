use super::context::Context;
use crate::context::ContextFactory;
use std::any::Any;

pub type BoxContext = Box<dyn Context + Send>;

pub type BoxContextFactory = Box<dyn ContextFactory<Context = BoxContext> + Send>;

pub fn boxed<T>(factory: T) -> BoxContextFactory
where
    T: ContextFactory + Send + 'static,
    <T as ContextFactory>::Context: Send,
{
    Box::new(FactoryWrapper(factory))
}

pub(crate) fn boxed_context<T>(context: T) -> BoxContext
where
    T: Context + Send + 'static,
{
    ContextWrapper::boxed(context)
}

struct FactoryWrapper<T: ContextFactory>(T);

impl<T> ContextFactory for FactoryWrapper<T>
where
    T: ContextFactory,
    <T as ContextFactory>::Context: Send + 'static,
{
    type Context = BoxContext;

    fn new_context(&self) -> Self::Context {
        let ctx = self.0.new_context();
        ContextWrapper::boxed(ctx)
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
    T: Context + Send + 'static;

impl<T> ContextWrapper<T>
where
    T: Context + Send + 'static,
{
    fn boxed(context: T) -> BoxContext {
        Box::new(ContextWrapper(context))
    }
}

impl<T> Context for ContextWrapper<T>
where
    T: Context + Send,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.0.as_any_mut()
    }
}
