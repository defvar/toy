//! Traits for Service.
//!

use crate::mpsc::Outgoing;
use crate::registry::PortType;
use crate::service_type::ServiceType;
use crate::task::TaskContext;
use std::any;
use std::fmt::{Debug, Error, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use toy_pack::schema::{Schema, SchemaVisitor, StructVisitor};

pub fn fn_service<F, Ctx, Req, Fut, Err, Pt>(
    tp: ServiceType,
    f: F,
    port: Pt,
) -> FnService<F, Ctx, Req, Fut, Err, Pt>
where
    F: FnMut(TaskContext, Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<ServiceContext<Ctx>, Err>> + Send,
    Pt: FnPortType,
{
    let _ = port;
    FnService {
        tp,
        f,
        _t: PhantomData,
    }
}

pub fn fn_service_factory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>(
    f: F,
    ctx_f: CtxF,
    port: Pt,
) -> FnServiceFactory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>
where
    F: Fn(ServiceType, Pt) -> Fut,
    Fut: Future<Output = Result<S, InitErr>> + Send,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> CtxFut,
    CtxFut: Future<Output = Result<S::Context, InitErr>> + Send,
    Cfg: Schema,
    Pt: FnPortType,
{
    FnServiceFactory {
        f,
        ctx_f,
        port,
        _t: PhantomData,
    }
}

pub trait ServiceFactory {
    type Future: Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service: Service<Request = Self::Request, Error = Self::Error, Context = Self::Context>;
    type CtxFuture: Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context;
    type Config: Schema;
    type Request;
    type Error;
    type InitError;

    fn new_service(&self, tp: ServiceType) -> Self::Future;

    fn new_context(&self, tp: ServiceType, config: Self::Config) -> Self::CtxFuture;
}

/// A value for reporting the current context to the executor.
/// The executor changes its behavior depending on this value.
pub enum ServiceContext<T> {
    /// processed request and waiting next request.
    Ready(T),

    /// complete service. not waiting next request.
    Complete(T),

    /// force next loop. not receiving next request.
    Next(T),
}

impl<T> ServiceContext<T> {
    pub fn into(self) -> T {
        match self {
            ServiceContext::Ready(c) => c,
            ServiceContext::Complete(c) => c,
            ServiceContext::Next(c) => c,
        }
    }
}

pub trait Service {
    type Context;
    type Request;
    type Future: Future<Output = Result<ServiceContext<Self::Context>, Self::Error>> + Send;
    type UpstreamFinishFuture: Future<Output = Result<ServiceContext<Self::Context>, Self::Error>>
        + Send;
    type UpstreamFinishAllFuture: Future<Output = Result<ServiceContext<Self::Context>, Self::Error>>
        + Send;
    type Error;

    fn port_type() -> PortType {
        PortType::flow()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future;

    fn started(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
    ) -> ServiceContext<Self::Context> {
        let _ = task_ctx;
        ServiceContext::Ready(ctx)
    }

    fn completed(&mut self, task_ctx: TaskContext, ctx: Self::Context) {
        let _ = task_ctx;
        let _ = ctx;
    }

    fn upstream_finish(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture;

    fn upstream_finish_all(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture;
}

pub struct FnService<F, Ctx, Req, Fut, Err, Pt>
where
    F: FnMut(TaskContext, Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<ServiceContext<Ctx>, Err>> + Send,
    Pt: FnPortType,
{
    tp: ServiceType,
    f: F,
    _t: PhantomData<(Ctx, Req, Fut, Err, Pt)>,
}

impl<F, Ctx, Req, Fut, Err, Pt> Service for FnService<F, Ctx, Req, Fut, Err, Pt>
where
    F: FnMut(TaskContext, Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<ServiceContext<Ctx>, Err>> + Send,
    Ctx: Send,
    Err: Send,
    Pt: FnPortType,
{
    type Context = Ctx;
    type Request = Req;
    type Future = Fut;
    type UpstreamFinishFuture = Ready<Result<ServiceContext<Ctx>, Err>>;
    type UpstreamFinishAllFuture = Ready<Result<ServiceContext<Ctx>, Err>>;
    type Error = Err;

    fn port_type() -> PortType {
        Pt::port_type()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        (self.f)(task_ctx, ctx, req, tx)
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture {
        ok(ServiceContext::Ready(ctx))
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        ok(ServiceContext::Complete(ctx))
    }
}

impl<F, Ctx, Req, Fut, Err, Pt> Debug for FnService<F, Ctx, Req, Fut, Err, Pt>
where
    F: FnMut(TaskContext, Ctx, Req, Outgoing<Req, Err>) -> Fut,
    Fut: Future<Output = Result<ServiceContext<Ctx>, Err>> + Send,
    Pt: FnPortType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "FnService {{ service_type:{:?} }}", self.tp)
    }
}

pub struct FnServiceFactory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>
where
    F: Fn(ServiceType, Pt) -> Fut,
    Fut: Future<Output = Result<S, InitErr>> + Send,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> CtxFut,
    CtxFut: Future<Output = Result<S::Context, InitErr>> + Send,
    Cfg: Schema,
    Pt: FnPortType,
{
    f: F,
    ctx_f: CtxF,
    port: Pt,
    _t: PhantomData<Cfg>,
}

impl<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt> ServiceFactory
    for FnServiceFactory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>
where
    F: Fn(ServiceType, Pt) -> Fut,
    Fut: Future<Output = Result<S, InitErr>> + Send,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> CtxFut,
    CtxFut: Future<Output = Result<S::Context, InitErr>> + Send,
    Cfg: Schema,
    Pt: FnPortType,
{
    type Future = Fut;
    type Service = S;
    type CtxFuture = CtxFut;
    type Context = S::Context;
    type Config = Cfg;
    type Request = S::Request;
    type Error = S::Error;
    type InitError = InitErr;

    fn new_service(&self, tp: ServiceType) -> Self::Future {
        let port = self.port;
        (self.f)(tp, port)
    }

    fn new_context(&self, tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        (self.ctx_f)(tp, config)
    }
}

impl<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt> Clone
    for FnServiceFactory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>
where
    F: Fn(ServiceType, Pt) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>> + Send,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> CtxFut + Clone,
    CtxFut: Future<Output = Result<S::Context, InitErr>> + Send,
    Cfg: Schema,
    Pt: FnPortType,
{
    fn clone(&self) -> Self {
        FnServiceFactory {
            f: self.f.clone(),
            ctx_f: self.ctx_f.clone(),
            port: self.port,
            _t: PhantomData,
        }
    }
}

impl<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt> Debug
    for FnServiceFactory<F, Fut, S, InitErr, CtxF, CtxFut, Cfg, Pt>
where
    F: Fn(ServiceType, Pt) -> Fut + Clone,
    Fut: Future<Output = Result<S, InitErr>> + Send,
    S: Service,
    CtxF: Fn(ServiceType, Cfg) -> CtxFut,
    CtxFut: Future<Output = Result<S::Context, InitErr>> + Send,
    Cfg: Schema,
    Pt: FnPortType,
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
    type Future = Ready<Result<ServiceContext<NoopContext>, ()>>;
    type UpstreamFinishFuture = Ready<Result<ServiceContext<NoopContext>, ()>>;
    type UpstreamFinishAllFuture = Ready<Result<ServiceContext<NoopContext>, ()>>;
    type Error = ();

    fn handle(
        &mut self,
        _task_ctx: TaskContext,
        _ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        ok(ServiceContext::Complete(NoopContext))
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        _ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture {
        ok(ServiceContext::Ready(NoopContext))
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        _ctx: Self::Context,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        ok(ServiceContext::Complete(NoopContext))
    }
}

#[derive(Debug, Clone)]
pub struct NoopServiceConfig;

impl Schema for NoopServiceConfig {
    fn scan<V>(_name: &str, mut visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        let v = visitor.struct_visitor("NoopServiceConfig")?;
        v.end()
    }
}

pub struct NoopServiceFactory;

impl ServiceFactory for NoopServiceFactory {
    type Future = Ready<Result<NoopService, ()>>;
    type Service = NoopService;
    type CtxFuture = Ready<Result<NoopContext, ()>>;
    type Context = NoopContext;
    type Config = NoopServiceConfig;
    type Request = ();
    type Error = ();
    type InitError = ();

    fn new_service(&self, tp: ServiceType) -> Self::Future {
        tracing::warn!(
            "create noop service. not found service? service_type: {:?}",
            tp
        );
        ok(NoopService)
    }

    fn new_context(&self, _tp: ServiceType, _config: Self::Config) -> Self::CtxFuture {
        ok(NoopContext)
    }
}

pub fn ok<T, E>(t: T) -> Ready<Result<T, E>> {
    Ready(Some(Ok(t)))
}

pub fn ready<T>(t: T) -> Ready<T> {
    Ready(Some(t))
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

pub trait FnPortType: Copy {
    fn port_type() -> PortType;
}

#[derive(Debug, Clone, Copy)]
pub struct SourcePort;
#[derive(Debug, Clone, Copy)]
pub struct FlowPort;
#[derive(Debug, Clone, Copy)]
pub struct SinkPort;

impl FnPortType for SourcePort {
    fn port_type() -> PortType {
        PortType::source()
    }
}

impl FnPortType for FlowPort {
    fn port_type() -> PortType {
        PortType::flow()
    }
}

impl FnPortType for SinkPort {
    fn port_type() -> PortType {
        PortType::sink()
    }
}
