use crate::channel::Outgoing;
use futures::Future;
use std::marker::PhantomData;

pub fn fn_service<F, Ctx, Req, Fut, Err>(f: F) -> FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req, Outgoing<Req>) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
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

pub struct FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req, Outgoing<Req>) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
{
    f: F,
    _t: PhantomData<(Ctx, Req)>,
}

impl<F, Ctx, Req, Fut, Err> Service for FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req, Outgoing<Req>) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
{
    type Context = Ctx;
    type Request = Req;
    type Future = Fut;
    type Error = Err;

    fn handle(
        &mut self,
        ctx: &mut Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        (self.f)(ctx, req, tx)
    }
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

pub trait Service {
    type Context;
    type Request;
    type Future: Future<Output = Result<(), Self::Error>>;
    type Error;

    fn handle(
        &mut self,
        ctx: &mut Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request>,
    ) -> Self::Future;

    fn started(&mut self, _ctx: &mut Self::Context) {}

    fn completed(&mut self, _ctx: &mut Self::Context) {}
}
