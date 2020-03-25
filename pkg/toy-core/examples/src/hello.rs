use futures::executor::block_on;
use futures::future::ok;
use log::info;
use std::thread;
use toy_core::channel::Outgoing;
use toy_core::data::{Frame, Map, Value};
use toy_core::error::ServiceError;
use toy_core::executor::DefaultExecutor;
use toy_core::factory;
use toy_core::graph::Graph;
use toy_core::registry::{Registry, ServiceSpawnerExt};
use toy_core::service;
use toy_core::ServiceType;
use toy_pack_derive::*;

#[derive(Clone, Debug, Default)]
pub struct ServiceContext;
#[derive(Clone, Debug, Default, UnPack)]
pub struct ServiceContextConfig {}

#[derive(Clone, Debug, Default)]
pub struct ServiceContext2 {
    count: u32,
}
#[derive(Clone, Debug, Default, UnPack)]
pub struct ServiceContext2Config {
    uri: String,
    prop1: u32,
}

struct ServiceContextFactory;

impl ServiceContextFactory {
    fn new_context<T: Into<ServiceType>>(
        _id: T,
        _config: ServiceContextConfig,
    ) -> Result<ServiceContext, ServiceError> {
        Ok(ServiceContext)
    }
}

struct ServiceContext2Factory;

impl ServiceContext2Factory {
    fn new_context<T: Into<ServiceType>>(
        _id: T,
        config: ServiceContext2Config,
    ) -> Result<ServiceContext2, ServiceError> {
        Ok(ServiceContext2 {
            count: config.prop1,
        })
    }
}

async fn service_3(
    mut ctx: ServiceContext2,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext2, ServiceError> {
    match req.value() {
        Value::U32(v) => ctx.count += *v,
        _ => (),
    };
    info!("service3 receive {:?}, ctx:{:?}", req, ctx);
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
    let _ = tx.send(Ok(Frame::from(ctx.count))).await?;
    Ok(ctx)
}

async fn service_1(
    ctx: ServiceContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext, ServiceError> {
    info!("service1 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send(Ok(Frame::from(1u32))).await?;
    let _ = tx.send(Ok(Frame::from(2u32))).await?;
    let _ = tx.send(Ok(Frame::from(3u32))).await?;
    let _ = tx.send(Ok(Frame::from(4u32))).await?;
    Ok(ctx)
}

async fn unboxed() -> Result<(), ()> {
    let c = Registry::new(
        "1",
        factory!(
            service_1,
            ServiceContextConfig,
            ServiceContextFactory::new_context
        ),
    )
    .service(
        "2",
        factory!(
            service_2,
            ServiceContext2Config,
            ServiceContext2Factory::new_context
        ),
    )
    .service(
        "3",
        factory!(
            service_3,
            ServiceContext2Config,
            ServiceContext2Factory::new_context
        ),
    );

    let g = graph();
    let e = DefaultExecutor::new(g);
    let _ = e.run(c, Frame::default()).await;

    Ok(())
}

fn graph() -> Graph {
    let mut s1 = Map::new();
    s1.insert("type".to_string(), Value::from("1".to_string()));
    s1.insert("uri".to_string(), Value::from("a".to_string()));
    s1.insert("prop1".to_string(), Value::from(0u32));
    s1.insert("wires".to_string(), Value::from("c"));
    let s1 = Value::from(s1);

    let mut s2 = Map::new();
    s2.insert("type".to_string(), Value::from("2".to_string()));
    s2.insert("uri".to_string(), Value::from("b".to_string()));
    s2.insert("prop1".to_string(), Value::from(0u32));
    s2.insert("wires".to_string(), Value::from("c"));
    let s2 = Value::from(s2);

    let mut s3 = Map::new();
    s3.insert("type".to_string(), Value::from("3".to_string()));
    s3.insert("uri".to_string(), Value::from("c".to_string()));
    s3.insert("prop1".to_string(), Value::from(0u32));
    s3.insert("wires".to_string(), Value::None);
    let s3 = Value::from(s3);

    let seq = Value::Seq(vec![s1, s2, s3]);

    let mut services = Map::new();
    services.insert("services".to_string(), seq);

    Graph::from(Value::Map(services)).unwrap()
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
    // info!("{:?}", graph());
}
