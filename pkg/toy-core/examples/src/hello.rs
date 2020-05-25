use futures::executor::{block_on, ThreadPool};
use futures::FutureExt;
use log::info;
use std::future::Future;
use std::thread;
use toy_core::prelude::*;
use toy_core::registry::{app, plugin};
use toy_pack_derive::*;

struct FutureRsRuntime {
    pool: ThreadPool,
}

impl AsyncRuntime for FutureRsRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.pool.spawn_ok(future.map(|_| ()));
    }
}

#[derive(Debug)]
pub struct ServiceContext;
#[derive(Clone, Debug, Default, Unpack)]
pub struct ServiceContextConfig {}

#[derive(Debug)]
pub struct ServiceContext2 {
    count: u32,
}
#[derive(Clone, Debug, Default, Unpack)]
pub struct ServiceContext2Config {
    uri: String,
    prop1: u32,
}

#[derive(Debug)]
pub struct ServiceContext3 {
    count: u32,
}
#[derive(Clone, Debug, Default, Unpack)]
pub struct ServiceContext3Config {
    uri: String,
    prop1: u32,
}

fn new_context(
    _tp: ServiceType,
    _config: ServiceContextConfig,
) -> Result<ServiceContext, ServiceError> {
    Ok(ServiceContext)
}

fn new_context2(
    _tp: ServiceType,
    config: ServiceContext2Config,
) -> Result<ServiceContext2, ServiceError> {
    Ok(ServiceContext2 {
        count: config.prop1,
    })
}

fn new_context3(
    _tp: ServiceType,
    config: ServiceContext3Config,
) -> Result<ServiceContext3, ServiceError> {
    Ok(ServiceContext3 {
        count: config.prop1,
    })
}

async fn service_3(
    mut ctx: ServiceContext3,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext3, ServiceError> {
    match req.value() {
        Value::U32(v) => ctx.count += *v,
        _ => (),
    };
    info!("service3 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send_ok(Frame::default()).await?;
    Ok(ctx)
}

async fn service_2(
    mut ctx: ServiceContext2,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext2, ServiceError> {
    ctx.count += 1;
    info!("service2 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send_ok(Frame::from(ctx.count)).await?;
    Ok(ctx)
}

async fn service_1(
    ctx: ServiceContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext, ServiceError> {
    info!("service1 receive {:?}, ctx:{:?}", req, ctx);
    let _ = tx.send_ok(Frame::from(1u32)).await?;
    let _ = tx.send_ok(Frame::from(2u32)).await?;
    let _ = tx.send_ok(Frame::from(3u32)).await?;
    let _ = tx.send_ok(Frame::from(4u32)).await?;
    Ok(ctx)
}

async fn unboxed() -> Result<(), ()> {
    let c = plugin(
        "example",
        "1",
        factory!(service_1, ServiceContextConfig, new_context),
    )
    .service(
        "2",
        factory!(service_2, ServiceContext2Config, new_context2),
    );

    let c1 = plugin(
        "example",
        "3",
        factory!(service_3, ServiceContext3Config, new_context3),
    );

    let a = app(c);
    let a = a.plugin(c1);
    log::debug!("{:?}", a);

    let rt = FutureRsRuntime {
        pool: ThreadPool::new().unwrap(),
    };
    let g = graph();
    let e = Executor::new(&rt, g);
    let _ = e.run(&a, Frame::default()).await;

    Ok(())
}

fn graph() -> Graph {
    let s1 = map_value! {
        "type" => "1",
        "uri" => "a",
        "prop1" => 0u32,
        "wires" => "c"
    };

    let s2 = map_value! {
        "type" => "2",
        "uri" => "b",
        "prop1" => 0u32,
        "wires" => "c"
    };

    let s3 = map_value! {
        "type" => "3",
        "uri" => "c",
        "prop1" => 0u32,
        "wires" => Option::<String>::None
    };

    let seq = Value::Seq(vec![s1, s2, s3]);

    let mut services = Map::new();
    services.insert("name".to_string(), Value::from("example"));
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
