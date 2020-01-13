use super::context_box::BoxContext;
use super::service::{Service, ServiceFactory};
use crate::channel::Outgoing;
use futures::future::{BoxFuture, FutureExt};

pub type BoxService<Req, Err> = Box<
    dyn Service<
        Request = Req,
        Error = Err,
        Future = BoxFuture<'static, Result<(), Err>>,
        Context = BoxContext,
    >,
>;

pub type BoxServiceFactory<Req, Err, InitErr> = Box<
    dyn ServiceFactory<
            Future = BoxFuture<'static, Result<BoxService<Req, Err>, InitErr>>,
            Service = BoxService<Req, Err>,
            Request = Req,
            Error = Err,
            InitError = InitErr,
            Context = BoxContext,
        > + Send
        + Sync,
>;

pub fn boxed<T>(factory: T) -> BoxServiceFactory<T::Request, T::Error, T::InitError>
where
    T: ServiceFactory + Send + Sync + 'static,
    T::Future: Send,
    T::Context: 'static,
    T::Request: 'static,
    T::Error: 'static,
    T::InitError: 'static,
    <T::Service as Service>::Future: Send,
{
    Box::new(FactoryWrapper(factory))
}

pub struct FactoryWrapper<T: ServiceFactory>(T);

impl<T, Req, Err, InitErr> ServiceFactory for FactoryWrapper<T>
where
    T: ServiceFactory<Request = Req, Error = Err, InitError = InitErr>,
    T::Future: 'static + Send,
    T::Service: 'static,
    <T::Service as Service>::Future: 'static + Send,
    Err: 'static,
    InitErr: 'static,
    Req: 'static,
{
    type Future = BoxFuture<'static, Result<Self::Service, Self::InitError>>;
    type Service = BoxService<Req, Err>;
    type Context = BoxContext;
    type Request = Req;
    type Error = Err;
    type InitError = InitErr;

    fn new_handler(&self) -> Self::Future {
        Box::pin(
            self.0
                .new_handler()
                .map(|res| res.map(ServiceWrapper::boxed)),
        )
    }
}

struct ServiceWrapper<T>(T)
where
    T: Service;

impl<T> ServiceWrapper<T>
where
    T: Service + 'static,
    T::Future: 'static + Send,
{
    fn boxed(service: T) -> BoxService<T::Request, T::Error> {
        Box::new(ServiceWrapper(service))
    }
}

impl<T, Req, Err> Service for ServiceWrapper<T>
where
    T: Service<Request = Req, Error = Err>,
    T::Future: 'static + Send,
    T::Context: 'static,
{
    type Context = BoxContext;
    type Request = Req;
    type Future = BoxFuture<'static, Result<(), Err>>;
    type Error = Err;

    fn handle(
        &mut self,
        ctx: &mut Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        if let Some(ctx) = ctx.as_any_mut().downcast_mut::<T::Context>() {
            Box::pin(self.0.handle(ctx, req, tx))
        } else {
            panic!("context couldn't downcast to concrete type")
        }
    }
}

impl<T> ServiceFactory for Box<T>
where
    T: ServiceFactory + ?Sized,
{
    type Future = T::Future;
    type Service = T::Service;
    type Context = T::Context;
    type Request = T::Request;
    type Error = T::Error;
    type InitError = T::InitError;

    fn new_handler(&self) -> Self::Future {
        (**self).new_handler()
    }
}

impl<S> Service for Box<S>
where
    S: Service + ?Sized,
{
    type Context = S::Context;
    type Request = S::Request;
    type Future = S::Future;
    type Error = S::Error;

    fn handle(
        &mut self,
        ctx: &mut Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        (**self).handle(ctx, req, tx)
    }
}
