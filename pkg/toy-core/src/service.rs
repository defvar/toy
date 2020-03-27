use crate::channel::Outgoing;
use crate::service_type::ServiceType;
use log::warn;
use std::any;
use std::fmt::{Debug, Error, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn fn_service<F, Ctx, Req, Fut, Err>(tp: ServiceType, f: F) -> FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    FnService {
        tp,
        f,
        _t: PhantomData,
    }
}

pub fn fn_service_factory<F, Fut, S, InitErr, CtxF, Cfg>(
    f: F,
    ctx_f: CtxF,
) -> FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceType) -> Fut,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> Result<S::Context, InitErr>,
{
    FnServiceFactory {
        f,
        ctx_f,
        _t: PhantomData,
    }
}

pub trait ServiceFactory {
    type Future: Future<Output = Result<Self::Service, Self::InitError>>;
    type Service: Service<Request = Self::Request, Error = Self::Error, Context = Self::Context>;
    type Context;
    type Config;
    type Request;
    type Error;
    type InitError;

    fn new_service(&self, tp: ServiceType) -> Self::Future;

    fn new_context(
        &self,
        tp: ServiceType,
        config: Self::Config,
    ) -> Result<Self::Context, Self::InitError>;
}

pub trait Service {
    type Context;
    type Request;
    type Future: Future<Output = Result<Self::Context, Self::Error>>;
    type Error;

    fn handle(
        &mut self,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future;

    fn started(&mut self, ctx: Self::Context) -> Self::Context {
        ctx
    }

    fn completed(&mut self, ctx: Self::Context) -> Self::Context {
        ctx
    }
}

pub struct FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    tp: ServiceType,
    f: F,
    _t: PhantomData<(Ctx, Req, Fut, Err)>,
}

impl<F, Ctx, Req, Fut, Err> Service for FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    type Context = Ctx;
    type Request = Req;
    type Future = Fut;
    type Error = Err;

    fn handle(
        &mut self,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        (self.f)(ctx, req, tx)
    }
}

impl<F, Ctx, Req, Fut, Err> Debug for FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "FnService {{ service_id:{:?} }}", self.tp)
    }
}

pub struct FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceType) -> Fut,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> Result<S::Context, InitErr>,
{
    f: F,
    ctx_f: CtxF,
    _t: PhantomData<Cfg>,
}

impl<F, Fut, S, InitErr, CtxF, Cfg> ServiceFactory
    for FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceType) -> Fut,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> Result<S::Context, InitErr>,
{
    type Future = Fut;
    type Service = S;
    type Context = S::Context;
    type Config = Cfg;
    type Request = S::Request;
    type Error = S::Error;
    type InitError = InitErr;

    fn new_service(&self, tp: ServiceType) -> Self::Future {
        (self.f)(tp)
    }

    fn new_context(
        &self,
        tp: ServiceType,
        config: Self::Config,
    ) -> Result<Self::Context, Self::InitError> {
        (self.ctx_f)(tp, config)
    }
}

impl<F, Fut, S, InitErr, CtxF, Cfg> Clone for FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceType) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> Result<S::Context, InitErr> + Clone,
{
    fn clone(&self) -> Self {
        FnServiceFactory {
            f: self.f.clone(),
            ctx_f: self.ctx_f.clone(),
            _t: PhantomData,
        }
    }
}

impl<F, Fut, S, InitErr, CtxF, Cfg> Debug for FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceType) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> Result<S::Context, InitErr>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "FnServiceFactory {{ S:{:?} }}",
            any::type_name::<S::Future>(),
        )
    }
}

////////////////////////////////////////////

pub struct NoopContext;

pub struct NoopService;

impl Service for NoopService {
    type Context = NoopContext;
    type Request = ();
    type Future = Ready<Result<NoopContext, ()>>;
    type Error = ();

    fn handle(
        &mut self,
        _ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        ok(NoopContext)
    }
}

pub struct NoopServiceFactory;

impl ServiceFactory for NoopServiceFactory {
    type Future = Ready<Result<NoopService, ()>>;
    type Service = NoopService;
    type Context = NoopContext;
    type Config = ();
    type Request = ();
    type Error = ();
    type InitError = ();

    fn new_service(&self, tp: ServiceType) -> Self::Future {
        warn!(
            "create noop service. not found service? service id: {:?}",
            tp
        );
        ok(NoopService)
    }

    fn new_context(
        &self,
        _tp: ServiceType,
        _config: Self::Config,
    ) -> Result<Self::Context, Self::InitError> {
        Ok(NoopContext)
    }
}

pub fn ok<T, E>(t: T) -> Ready<Result<T, E>> {
    Ready(Some(Ok(t)))
}

#[derive(Debug, Clone)]
pub struct Ready<T>(Option<T>);

impl<T> Ready<T> {
    /// Unwraps the value from this immediately ready future.
    #[inline]
    pub fn into_inner(mut self) -> T {
        self.0.take().unwrap()
    }
}

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().unwrap())
    }
}
