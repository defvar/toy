use super::context_box::BoxContext;
use super::service::{Handler2, ServiceFactory};
use futures::future::{BoxFuture, FutureExt};

pub type BoxHandler2<Req, Err> = Box<
    dyn Handler2<
        Request = Req,
        Error = Err,
        Future = BoxFuture<'static, Result<(), Err>>,
        Context = BoxContext,
    >,
>;

pub type BoxServiceFactory<Req, Err, InitErr> = Box<
    dyn ServiceFactory<
        Future = BoxFuture<'static, Result<BoxHandler2<Req, Err>, InitErr>>,
        Handler = BoxHandler2<Req, Err>,
        Request = Req,
        Error = Err,
        InitError = InitErr,
        Context = BoxContext,
    >,
>;

pub fn boxed<T>(factory: T) -> BoxServiceFactory<T::Request, T::Error, T::InitError>
where
    T: ServiceFactory + 'static,
    T::Future: Send,
    T::Context: 'static,
    T::Request: 'static,
    T::Error: 'static,
    T::InitError: 'static,
    <T::Handler as Handler2>::Future: Send,
{
    Box::new(FactoryWrapper(factory))
}

struct FactoryWrapper<T: ServiceFactory>(T);

impl<T, Req, Err, InitErr> ServiceFactory for FactoryWrapper<T>
where
    T: ServiceFactory<Request = Req, Error = Err, InitError = InitErr>,
    T::Future: 'static + Send,
    T::Handler: 'static,
    <T::Handler as Handler2>::Future: 'static + Send,
    Err: 'static,
    InitErr: 'static,
    Req: 'static,
{
    type Future = BoxFuture<'static, Result<Self::Handler, Self::InitError>>;
    type Handler = BoxHandler2<Req, Err>;
    type Context = BoxContext;
    type Request = Req;
    type Error = Err;
    type InitError = InitErr;

    fn new_handler(&self) -> Self::Future {
        Box::pin(
            self.0
                .new_handler()
                .map(|res| res.map(Handler2Wrapper::boxed)),
        )
    }
}

struct Handler2Wrapper<T>(T)
where
    T: Handler2;

impl<T> Handler2Wrapper<T>
where
    T: Handler2 + 'static,
    T::Future: 'static + Send,
{
    fn boxed(handler: T) -> BoxHandler2<T::Request, T::Error> {
        Box::new(Handler2Wrapper(handler))
    }
}

impl<T, Req, Err> Handler2 for Handler2Wrapper<T>
where
    T: Handler2<Request = Req, Error = Err>,
    T::Future: 'static + Send,
    T::Context: 'static,
{
    type Context = BoxContext;
    type Request = Req;
    type Future = BoxFuture<'static, Result<(), Err>>;
    type Error = Err;

    fn handle(&mut self, ctx: &mut Self::Context, req: Self::Request) -> Self::Future {
        if let Some(ctx) = ctx.as_any_mut().downcast_mut::<T::Context>() {
            Box::pin(self.0.handle(ctx, req))
        } else {
            panic!("context couldn't downcast to concrete type")
        }
    }
}

impl<S> Handler2 for Box<S>
where
    S: Handler2 + ?Sized,
{
    type Context = S::Context;
    type Request = S::Request;
    type Future = S::Future;
    type Error = S::Error;

    fn handle(&mut self, ctx: &mut Self::Context, req: Self::Request) -> Self::Future {
        (**self).handle(ctx, req)
    }
}
