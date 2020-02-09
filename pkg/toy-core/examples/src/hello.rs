use futures::executor::block_on;
use futures::future::ok;
use log::info;
use std::thread;
use toy_core::channel::Outgoing;
use toy_core::data::Frame;
use toy_core::error::ServiceError;
use toy_core::executor::DefaultExecutor;
use toy_core::registry::{Registry, ServiceSpawner, ServiceSpawnerExt};
use toy_core::service;
use toy_core::service_id::ServiceId;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext2 {
    count: u32,
}

struct ServiceContextFactory;

impl ServiceContextFactory {
    fn new_context<T: Into<ServiceId>>(_id: T) -> ServiceContext {
        ServiceContext
    }
}

struct ServiceContext2Factory;

impl ServiceContext2Factory {
    fn new_context<T: Into<ServiceId>>(_id: T) -> ServiceContext2 {
        ServiceContext2::default()
    }
}

async fn service_3(
    ctx: ServiceContext2,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext2, ServiceError> {
    info!("service3 !");
    let _ = tx.send(Ok(Frame::default())).await?;
    Ok(ctx)
}

async fn service_2(
    mut ctx: ServiceContext2,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext2, ServiceError> {
    ctx.count += 1;
    info!("service2 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send(Ok(Frame::default())).await?;
    Ok(ctx)
}

async fn service_1(
    ctx: ServiceContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext, ServiceError> {
    info!("service1 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send(Ok(Frame::from(1))).await?;
    let _ = tx.send(Ok(Frame::from(2))).await?;
    let _ = tx.send(Ok(Frame::from(3))).await?;
    let _ = tx.send(Ok(Frame::from(4))).await?;
    Ok(ctx)
}

macro_rules! factory {
    ($f:expr, $ctx_f:expr) => {{
        || {
            service::fn_service_factory(
                |id: ServiceId| ok::<_, ServiceError>(service::fn_service(id, $f)),
                |id: ServiceId| $ctx_f(id),
            )
        }
    }};
}

async fn unboxed() -> Result<(), ()> {
    let c = Registry::new(
        "1".into(),
        factory!(service_1, ServiceContextFactory::new_context),
    )
    .service(
        "2".into(),
        factory!(service_2, ServiceContext2Factory::new_context),
    )
    .service(
        "3".into(),
        factory!(service_3, ServiceContext2Factory::new_context),
    );

    let mut e = DefaultExecutor::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    // need to reverse ....
    let _ = c.spawn("3".into(), &mut e);
    let _ = c.spawn("2".into(), &mut e);
    let _ = c.spawn("1".into(), &mut e);
    let _ = e.run(Frame::default()).await;

    Ok(())
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
    let _ = block_on(unboxed());
}
