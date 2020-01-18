use futures::executor::block_on;
use futures::future::ok;
use log::info;
use std::any::Any;
use std::thread;
use toy_core::channel::Outgoing;
use toy_core::context::{Context, ContextFactory};
use toy_core::context_box;
use toy_core::data::Frame;
use toy_core::error::MessagingError;
use toy_core::flow::Flow;
use toy_core::registry::Registry;
use toy_core::service;
use toy_core::service_box;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext2 {
    count: u32,
}

impl Context for ServiceContext {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Context for ServiceContext2 {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct ServiceContextFactory;

impl ContextFactory for ServiceContextFactory {
    type Context = ServiceContext;

    fn new_context(&self) -> Self::Context {
        ServiceContext
    }
}

struct ServiceContext2Factory;

impl ContextFactory for ServiceContext2Factory {
    type Context = ServiceContext2;

    fn new_context(&self) -> Self::Context {
        ServiceContext2 { count: 0 }
    }
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    info!("-----------------------------------");
    info!("main thread {:?}", thread::current().id());

    let factory = service::fn_service_factory(|| {
        ok::<_, MessagingError>(service::fn_service(
            |ctx: ServiceContext, _req: Frame, mut tx: Outgoing<Frame, MessagingError>| {
                async move {
                    let _ = tx.send(Ok(Frame::from(1))).await?;
                    let _ = tx.send(Ok(Frame::from(2))).await?;
                    let _ = tx.send(Ok(Frame::from(3))).await?;
                    let _ = tx.send(Ok(Frame::from(4))).await?;
                    Ok(ctx)
                }
            },
        ))
    });

    let factory2 = service::fn_service_factory(|| {
        ok::<_, MessagingError>(service::fn_service(
            |mut ctx: ServiceContext2, req: Frame, _tx: Outgoing<Frame, MessagingError>| {
                async move {
                    ctx.count += 1;
                    info!("receive {:?}, ctx:{:?}", req, ctx);
                    Ok(ctx)
                }
            },
        ))
    });

    let mut regi = Registry::new();
    regi.set(
        "aaa",
        service_box::boxed(factory),
        context_box::boxed(ServiceContextFactory),
    );
    regi.set(
        "bbb",
        service_box::boxed(factory2),
        context_box::boxed(ServiceContext2Factory),
    );

    let ok = block_on(Flow::new().start(regi, vec!["aaa".to_string(), "bbb".to_string()]));
    info!("{:?}", ok);
}
