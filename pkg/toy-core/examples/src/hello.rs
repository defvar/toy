use toy_core::prelude::*;
use toy_core::registry::{app, plugin, PortType};
use toy_core::supervisor::{Request, Supervisor};
use toy_pack_derive::*;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug)]
pub struct PublishContext;
#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct PublishConfig {}

#[derive(Debug)]
pub struct ReceiveContext {
    prop1: u32,
}

#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct ReceiveConfig {
    prop1: u32,
}

#[derive(Debug)]
pub struct AccumulateContext {
    count: u32,
}
#[derive(Clone, Debug, Default, Unpack, Schema)]
pub struct AccumulateConfig {
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
    config: ReceiveConfig,
) -> Result<ReceiveContext, ServiceError> {
    Ok(ReceiveContext {
        prop1: config.prop1,
    })
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
) -> Result<ServiceContext<AccumulateContext>, ServiceError> {
    match req.value().unwrap() {
        Value::U32(v) => ctx.count += *v,
        _ => (),
    };
    tracing::info!(
        "accumulate value:{:?} from port:{:?} -> ctx:{:?}",
        req,
        req.port(),
        ctx
    );
    let _ = tx.send_ok(Frame::default()).await?;
    Ok(ServiceContext::Ready(ctx))
}

async fn receive(
    ctx: ReceiveContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<ReceiveContext>, ServiceError> {
    tracing::info!("receive/{:?}. send value {:?}.", ctx.prop1, req);
    let _ = tx.send_ok(req).await?;
    Ok(ServiceContext::Ready(ctx))
}

async fn publish(
    ctx: PublishContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<PublishContext>, ServiceError> {
    tracing::info!("publish");

    let _ = tx.send_ok(Frame::from(1u32)).await?;
    let _ = tx.send_ok(Frame::from(2u32)).await?;
    let _ = tx.send_ok(Frame::from(3u32)).await?;
    let _ = tx.send_ok(Frame::from(4u32)).await?;

    let _ = tx.send_ok_to(1, Frame::from(1u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(2u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(3u32)).await?;
    let _ = tx.send_ok_to(1, Frame::from(4u32)).await?;

    Ok(ServiceContext::Complete(ctx))
}

fn graph() -> Graph {
    let s1 = map_value! {
        "type" => "example.pub",
        "uri" => "ex/pub",
        "config" => map_value! {
            "prop1" => 0u32,
        },
        "wires" => seq_value!["ex/rec/1", "ex/rec/2"]
    };

    let s2 = map_value! {
        "type" => "example.rec",
        "uri" => "ex/rec/1",
        "config" => map_value! {
            "prop1" => 1u32,
        },
        "wires" => "ex/acc"
    };

    let s3 = map_value! {
        "type" => "example.rec",
        "uri" => "ex/rec/2",
        "config" => map_value! {
            "prop1" => 2u32,
        },
        "wires" => "ex/acc"
    };

    let s4 = map_value! {
        "type" => "example.acc",
        "uri" => "ex/acc",
        "config" => map_value! {
            "prop1" => 0u32,
        },
        "wires" => Option::<String>::None
    };

    let seq = Value::Seq(vec![s1, s2, s3, s4]);

    let mut services = Map::new();
    services.insert("name".to_string(), Value::from("example"));
    services.insert("services".to_string(), seq);

    let r = Graph::from(Value::Map(services)).unwrap();
    tracing::info!("{:?}", r);
    r
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let c = plugin(
        "example",
        "pub",
        PortType::fan_out_source(2),
        factory!(publish, PublishConfig, new_publish_context),
    )
    .with(
        "rec",
        PortType::flow(),
        factory!(receive, ReceiveConfig, new_receive_context),
    )
    .with(
        "acc",
        PortType::fan_in_sink(2),
        factory!(accumulate, AccumulateConfig, new_accumulate_context),
    );

    let app = app(c);
    tracing::debug!("{:?}", app);

    tracing::info!("-----------------------------------");
    tracing::info!("main thread");

    // runtime for supervisor
    let mut rt = toy_rt::RuntimeBuilder::new()
        .worker_threads(4)
        .thread_name("toy-worker")
        .build()
        .unwrap();

    let (sv, mut tx, mut rx) = Supervisor::new(toy_rt::Spawner, app);

    // supervisor start
    rt.spawn(async {
        let _ = sv.run().await;
    });

    tracing::info!("send task request to supervisor");
    let _ = rt.block_on(async {
        let (tx2, rx2) = toy_core::oneshot::channel();
        let _ = tx.send_ok(Request::Task(graph(), tx2)).await;
        let uuid = rx2.recv().await;
        log::info!("task:{:?}", uuid);
    });

    tracing::info!("waiting shutdown reply from supervisor");
    let _ = rt.block_on(async {
        rx.next().await;
    });
}
