use super::context_box::{self, BoxContext};
use super::service::{Service, ServiceFactory};
use crate::channel::Outgoing;
use crate::context::Context;
use crate::service_id::ServiceId;
use futures::future::{BoxFuture, FutureExt};

pub type BoxService<Req, Err> = Box<
    dyn Service<
            Request = Req,
            Error = Err,
            Future = BoxFuture<'static, Result<BoxContext, Err>>,
            Context = BoxContext,
        > + Send,
>;

pub type BoxServiceFactory<Req, Err, InitErr> = Box<
    dyn ServiceFactory<
            Future = BoxFuture<'static, Result<BoxService<Req, Err>, InitErr>>,
            Service = BoxService<Req, Err>,
            Request = Req,
            Error = Err,
            InitError = InitErr,
            Context = BoxContext,
        > + Send,
>;

pub fn boxed<T>(factory: T) -> BoxServiceFactory<T::Request, T::Error, T::InitError>
where
    T: ServiceFactory + Send + Sync + 'static,
    T::Future: Send,
    T::Context: 'static,
    T::Request: 'static,
    T::Error: 'static,
    T::InitError: 'static,
    T::Service: Send,
    <T::Service as Service>::Future: Send,
    <T::Service as Service>::Context: Context + Default + Send + 'static,
{
    Box::new(FactoryWrapper(factory))
}

struct FactoryWrapper<T: ServiceFactory>(T);

impl<T, Req, Err, InitErr> ServiceFactory for FactoryWrapper<T>
where
    T: ServiceFactory<Request = Req, Error = Err, InitError = InitErr>,
    T::Future: 'static + Send,
    T::Service: 'static + Send,
    <T::Service as Service>::Future: 'static + Send,
    <T::Service as Service>::Context: Context + Default + Send + 'static,
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

    fn new_service(&self, id: ServiceId) -> Self::Future {
        Box::pin(
            self.0
                .new_service(id)
                .map(|res| res.map(ServiceWrapper::boxed)),
        )
    }

    fn new_context(&self, id: ServiceId) -> Self::Context {
        Box::new(self.0.new_context(id))
    }
}

struct ServiceWrapper<T>(T)
where
    T: Service;

impl<T> ServiceWrapper<T>
where
    T: Service + 'static + Send,
    T::Context: Context + Default + Send + 'static,
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
    T::Context: Context + Send + Default + 'static,
{
    type Context = BoxContext;
    type Request = Req;
    type Future = BoxFuture<'static, Result<BoxContext, Err>>;
    type Error = Err;

    fn handle(
        &mut self,
        mut ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        if let Some(ctx) = ctx.as_any_mut().downcast_mut::<T::Context>() {
            let ctx = std::mem::replace(ctx, T::Context::default());
            Box::pin(self.0.handle(ctx, req, tx).map(|x| match x {
                Ok(ctx) => Ok(context_box::boxed_context(ctx)),
                Result::Err(e) => Result::Err(e),
            }))
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

    fn new_service(&self, id: ServiceId) -> Self::Future {
        (**self).new_service(id)
    }

    fn new_context(&self, id: ServiceId) -> Self::Context {
        (**self).new_context(id)
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
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        (**self).handle(ctx, req, tx)
    }
}
