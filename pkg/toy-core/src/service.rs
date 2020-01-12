use super::context::Context;
use super::context::ServiceContext;
use super::data::Frame;
use super::error::Error;
use super::error::MessagingError;
use futures::Future;
use std::marker::PhantomData;

pub trait Service:
    Handler<Request = Frame, Error = MessagingError, Future = (), Context = ServiceContext>
    + Send
    + Sync
{
}

pub fn fn_service<F, Ctx, Req, Fut, Err>(f: F) -> FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
{
    FnService { f, _t: PhantomData }
}

pub fn fn_service_factory<F, Fut, H, InitErr>(f: F) -> FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<H, InitErr>>,
    H: Handler2,
{
    FnServiceFactory { f }
}

pub struct FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
{
    f: F,
    _t: PhantomData<(Ctx, Req)>,
}

impl<F, Ctx, Req, Fut, Err> Handler2 for FnService<F, Ctx, Req, Fut, Err>
where
    F: FnMut(&mut Ctx, Req) -> Fut,
    Fut: Future<Output = Result<(), Err>>,
{
    type Context = Ctx;
    type Request = Req;
    type Future = Fut;
    type Error = Err;

    fn handle(&mut self, ctx: &mut Self::Context, req: Self::Request) -> Self::Future {
        (self.f)(ctx, req)
    }
}

pub trait ServiceFactory {
    type Future: Future<Output = Result<Self::Handler, Self::InitError>>;
    type Handler: Handler2<Request = Self::Request, Error = Self::Error, Context = Self::Context>;
    type Context;
    type Request;
    type Error;
    type InitError;

    fn new_handler(&self) -> Self::Future;
}

pub struct FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<H, InitErr>>,
{
    f: F,
}

impl<F, Fut, H, InitErr> ServiceFactory for FnServiceFactory<F, Fut, H, InitErr>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<H, InitErr>>,
    H: Handler2,
{
    type Future = Fut;
    type Handler = H;
    type Context = H::Context;
    type Request = H::Request;
    type Error = H::Error;
    type InitError = InitErr;

    fn new_handler(&self) -> Self::Future {
        (self.f)()
    }
}

pub trait Handler2 {
    type Context;
    type Request;
    type Future: Future<Output = Result<(), Self::Error>>;
    type Error;

    fn handle(&mut self, ctx: &mut Self::Context, req: Self::Request) -> Self::Future;

    fn started(&mut self, _ctx: &mut Self::Context) {}

    fn completed(&mut self, _ctx: &mut Self::Context) {}
}

pub trait Handler {
    type Request;
    type Error: Error;
    type Future;
    type Context: Context;

    fn handle(&self, ctx: &mut Self::Context, req: Self::Request) -> Self::Future;

    fn start(&mut self) {}

    fn completed(&mut self) {}
}
