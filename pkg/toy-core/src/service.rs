use crate::channel::Outgoing;
use crate::context::Context;
use crate::service_id::ServiceId;
use futures::future::Ready;
use futures::{future, Future};
use log::warn;
use std::any;
use std::fmt::{Debug, Error, Formatter};
use std::marker::PhantomData;

pub fn fn_service<F, Ctx, Req, Fut, Err>(id: ServiceId, f: F) -> FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    FnService {
        id,
        f,
        _t: PhantomData,
    }
}

pub fn fn_service_factory<F, Fut, S, InitErr, CtxF, Cfg>(
    f: F,
    ctx_f: CtxF,
) -> FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceId) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceId, Cfg) -> S::Context + Clone,
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

    fn new_service(&self, id: ServiceId) -> Self::Future;

    fn new_context(&self, id: ServiceId, config: Self::Config) -> Self::Context;
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
    id: ServiceId,
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
        write!(f, "FnService {{ service_id:{:?} }}", self.id)
    }
}

pub struct FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceId) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceId, Cfg) -> S::Context + Clone,
{
    f: F,
    ctx_f: CtxF,
    _t: PhantomData<Cfg>,
}

impl<F, Fut, S, InitErr, CtxF, Cfg> ServiceFactory
    for FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceId) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceId, Cfg) -> S::Context + Clone,
{
    type Future = Fut;
    type Service = S;
    type Context = S::Context;
    type Config = Cfg;
    type Request = S::Request;
    type Error = S::Error;
    type InitError = InitErr;

    fn new_service(&self, id: ServiceId) -> Self::Future {
        (self.f)(id)
    }

    fn new_context(&self, id: ServiceId, config: Self::Config) -> Self::Context {
        (self.ctx_f)(id, config)
    }
}

impl<F, Fut, S, InitErr, CtxF, Cfg> Clone for FnServiceFactory<F, Fut, S, InitErr, CtxF, Cfg>
where
    F: Fn(ServiceId) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceId, Cfg) -> S::Context + Clone,
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
    F: Fn(ServiceId) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>>,
    S: Service,
    CtxF: Fn(ServiceId, Cfg) -> S::Context + Clone,
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

impl Context for NoopContext {
    type Config = ();

    fn set_config(&mut self, _config: Self::Config) {}
}

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
        future::ready(Ok(NoopContext))
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

    fn new_service(&self, id: ServiceId) -> Self::Future {
        warn!(
            "create noop service. not found service? service id: {:?}",
            id
        );
        future::ready(Ok(NoopService))
    }

    fn new_context(&self, _id: ServiceId, _config: Self::Config) -> Self::Context {
        NoopContext
    }
}
