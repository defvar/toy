use futures::executor::{block_on, ThreadPool};
use futures::FutureExt;
use log::info;
use std::future::Future;
use std::thread;
use std::time::Duration;
use toy_core::prelude::*;
use toy_core::registry::{app, plugin};
use toy_core::supervisor::{Request, Supervisor};
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
pub struct PublishContext;
#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct PublishConfig {}

#[derive(Debug)]
pub struct ReceiveContext {}

#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct ReceiveConfig {
    uri: String,
    prop1: u32,
}

#[derive(Debug)]
pub struct AccumulateContext {
    count: u32,
}
#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct AccumulateConfig {
    uri: String,
    prop1: u32,
}

fn new_publish_context(
    _tp: ServiceType,
    _config: PublishConfig,
) -> Result<PublishContext, ServiceError> {
    Ok(PublishContext)
}

fn new_receive_context(
    _tp: ServiceType,
    _config: ReceiveConfig,
) -> Result<ReceiveContext, ServiceError> {
    Ok(ReceiveContext {})
}

fn new_accumulate_context(
    _tp: ServiceType,
    config: AccumulateConfig,
) -> Result<AccumulateContext, ServiceError> {
    Ok(AccumulateContext {
        count: config.prop1,
    })
}

async fn accumulate(
    mut ctx: AccumulateContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<AccumulateContext, ServiceError> {
    match req.value() {
        Value::U32(v) => ctx.count += *v,
        _ => (),
    };
    info!(
        "accumulate value:{:?} from port:{:?} -> ctx:{:?}",
        req,
        req.port(),
        ctx
    );
    let _ = tx.send_ok(Frame::default()).await?;
    Ok(ctx)
}

async fn receive(
    ctx: ReceiveContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ReceiveContext, ServiceError> {
    info!("receive and send value {:?}.", req);
    let _ = tx.send_ok(req).await?;
    Ok(ctx)
}

async fn publish(
    ctx: PublishContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<PublishContext, ServiceError> {
    info!("publish");
    let _ = tx.send_ok(Frame::from(1u32)).await?;
    let _ = tx.send_ok(Frame::from(2u32)).await?;
    let _ = tx.send_ok(Frame::from(3u32)).await?;
    let _ = tx.send_ok(Frame::from(4u32)).await?;

    let _ = tx.send_ok_to(1, Frame::from(1u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(2u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(3u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(4u32)).await?;

    Ok(ctx)
}

fn graph() -> Graph {
    let s1 = map_value! {
        "type" => "example.pub",
        "uri" => "ex/pub",
        "prop1" => 0u32,
        "wires" => seq_value!["ex/rec/1", "ex/rec/2"]
    };

    let s2 = map_value! {
        "type" => "example.rec",
        "uri" => "ex/rec/1",
        "prop1" => 0u32,
        "wires" => "ex/acc"
    };

    let s3 = map_value! {
        "type" => "example.rec",
        "uri" => "ex/rec/2",
        "prop1" => 0u32,
        "wires" => "ex/acc"
    };

    let s4 = map_value! {
        "type" => "example.acc",
        "uri" => "ex/acc",
        "prop1" => 0u32,
        "wires" => Option::<String>::None
    };

    let seq = Value::Seq(vec![s1, s2, s3, s4]);

    let mut services = Map::new();
    services.insert("name".to_string(), Value::from("example"));
    services.insert("services".to_string(), seq);

    let r = Graph::from(Value::Map(services)).unwrap();
    info!("{:?}", r);
    r
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let c = plugin(
        "example",
        "pub",
        factory!(publish, PublishConfig, new_publish_context),
    )
    .service("rec", factory!(receive, ReceiveConfig, new_receive_context))
    .service(
        "acc",
        factory!(accumulate, AccumulateConfig, new_accumulate_context),
    );

    let a = app(c);
    log::debug!("{:?}", a);

    info!("-----------------------------------");
    info!("main thread {:?}", thread::current().id());

    // runtime for services
    let service_rt = FutureRsRuntime {
        pool: ThreadPool::new().unwrap(),
    };
    // runtime for supervisor
    let sv_rt = FutureRsRuntime {
        pool: ThreadPool::new().unwrap(),
    };

    let (sv, mut tx, _) = Supervisor::new(service_rt, a);

    // supervisor start
    sv_rt.spawn(async {
        let _ = sv.run().await;
    });

    // send request to supervisor
    let _ = block_on(async {
        let _ = tx.send_ok(Request::Task(graph())).await;
    });

    // wait task end
    thread::sleep(Duration::from_secs(3));
}
