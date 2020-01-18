use crate::channel::Outgoing;
use futures::Future;
use std::marker::PhantomData;

pub fn fn_service<F, Ctx, Req, Fut, Err>(f: F) -> FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<Ctx, Err>>,
{
    FnService { f, _t: PhantomData }
}

pub fn fn_service_factory<F, Fut, H, InitErr>(f: F) -> FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<H, InitErr>>,
    H: Service,
{
    FnServiceFactory { f }
}

pub trait ServiceFactory {
    type Future: Future<Output = Result<Self::Service, Self::InitError>>;
    type Service: Service<Request = Self::Request, Error = Self::Error, Context = Self::Context>;
    type Context;
    type Request;
    type Error;
    type InitError;

    fn new_handler(&self) -> Self::Future;
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

pub struct FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<H, InitErr>>,
{
    f: F,
}

impl<F, Fut, H, InitErr> ServiceFactory for FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<H, InitErr>>,
    H: Service,
{
    type Future = Fut;
    type Service = H;
    type Context = H::Context;
    type Request = H::Request;
    type Error = H::Error;
    type InitError = InitErr;

    fn new_handler(&self) -> Self::Future {
        (self.f)()
    }
}

impl<F, Fut, H, InitErr> Clone for FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<H, InitErr>>,
    H: Service,
{
    fn clone(&self) -> Self {
        FnServiceFactory { f: self.f.clone() }
    }
}
