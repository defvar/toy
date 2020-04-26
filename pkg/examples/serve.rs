use toy_api_server::new_graph_registry;
use toy_core::prelude::*;
use toy_core::registry::Registry;
use toy_core::supervisor::Supervisor;
use toy_pack::UnPack;

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let mut sv_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("supervisor")
        .basic()
        .build()
        .unwrap();

    let api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .threaded()
        .core_threads(1)
        .max_threads(1)
        .build()
        .unwrap();

    let regi = Registry::new("hello1", factory!(hello, ServiceContextConfig, new_context))
        .service("hello2", factory!(hello, ServiceContextConfig, new_context));

    let service_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("service-worker")
        .threaded()
        .build()
        .unwrap();

    let (sv, tx, rx) = Supervisor::new(service_rt, regi);

    let g = new_graph_registry(vec![sample_graph("sample1"), sample_graph("sample2")]);
    let server = toy_api_server::Server::new(g);

    api_rt.spawn(async {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
    });
    sv_rt.block_on(async {
        let _ = sv.run().await;
    });

    // sv_rt.block_on(async {
    //     let _ = a.await;
    // })
}

#[derive(Debug)]
pub struct ServiceContext;

#[derive(Clone, Debug, Default, UnPack)]
pub struct ServiceContextConfig {}

fn new_context(
    _tp: ServiceType,
    _config: ServiceContextConfig,
) -> Result<ServiceContext, ServiceError> {
    Ok(ServiceContext)
}

async fn hello(
    ctx: ServiceContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext, ServiceError> {
    let _ = tx.send(Ok(Frame::from(1u32))).await?;
    Ok(ctx)
}

fn sample_graph(name: &'static str) -> Graph {
    let mut s1 = Map::new();
    s1.insert("type".to_string(), Value::from("hello1".to_string()));
    s1.insert("uri".to_string(), Value::from("a".to_string()));
    s1.insert("prop1".to_string(), Value::from(0u32));
    s1.insert("wires".to_string(), Value::from("b"));
    let s1 = Value::from(s1);

    let mut s2 = Map::new();
    s2.insert("type".to_string(), Value::from("hello2".to_string()));
    s2.insert("uri".to_string(), Value::from("b".to_string()));
    s2.insert("prop1".to_string(), Value::from(0u32));
    s2.insert("wires".to_string(), Value::from("c"));
    let s2 = Value::from(s2);

    let mut s3 = Map::new();
    s3.insert("type".to_string(), Value::from("hello1".to_string()));
    s3.insert("uri".to_string(), Value::from("c".to_string()));
    s3.insert("prop1".to_string(), Value::from(0u32));
    s3.insert("wires".to_string(), Value::None);
    let s3 = Value::from(s3);

    let seq = Value::Seq(vec![s1, s2, s3]);

    let mut services = Map::new();
    services.insert("name".to_string(), Value::from(name));
    services.insert("services".to_string(), seq);

    Graph::from(Value::Map(services)).unwrap()
}
