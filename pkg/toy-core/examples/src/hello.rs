use futures::executor::block_on;
use futures::future::ok;
use log::info;
use std::collections::HashMap;
use std::thread;
use toy_core::channel::Outgoing;
use toy_core::data::{Frame, Value};
use toy_core::error::ServiceError;
use toy_core::executor::DefaultExecutor;
use toy_core::factory;
use toy_core::graph::Graph;
use toy_core::registry::{Registry, ServiceSpawner, ServiceSpawnerExt};
use toy_core::service;
use toy_core::service_id::ServiceId;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext;
#[derive(Clone, Debug, Default)]
pub struct ServiceContextConfig;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext2 {
    count: u32,
}
#[derive(Clone, Debug, Default)]
pub struct ServiceContext2Config;

struct ServiceContextFactory;

impl ServiceContextFactory {
    fn new_context<T: Into<ServiceId>>(_id: T, _config: ServiceContextConfig) -> ServiceContext {
        ServiceContext
    }
}

struct ServiceContext2Factory;

impl ServiceContext2Factory {
    fn new_context<T: Into<ServiceId>>(_id: T, _config: ServiceContext2Config) -> ServiceContext2 {
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

async fn unboxed() -> Result<(), ()> {
    let c = Registry::new(
        "1".into(),
        factory!(
            service_1,
            ServiceContextConfig,
            ServiceContextFactory::new_context
        ),
    )
    .service(
        "2".into(),
        factory!(
            service_2,
            ServiceContext2Config,
            ServiceContext2Factory::new_context
        ),
    )
    .service(
        "3".into(),
        factory!(
            service_3,
            ServiceContext2Config,
            ServiceContext2Factory::new_context
        ),
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
    //    let _ = block_on(unboxed());
    let mut s1 = HashMap::new();
    s1.insert("kind".to_string(), Value::from("aaaa".to_string()));
    s1.insert("uri".to_string(), Value::from("http://aaaa".to_string()));
    s1.insert("prop1".to_string(), Value::from(1u32));
    let s1 = Value::from(s1);

    let mut s2 = HashMap::new();
    s2.insert("kind".to_string(), Value::from("bbbb".to_string()));
    s2.insert("uri".to_string(), Value::from("http://bbbb".to_string()));
    s2.insert("prop1".to_string(), Value::from(2u32));
    let s2 = Value::from(s2);

    let mut seq = Value::Seq(vec![s1, s2]);

    let mut services = HashMap::new();
    services.insert("services".to_string(), seq);

    let r = Graph::from(Value::Map(services));
    info!("{:?}", r);
}
