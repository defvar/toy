use toy_api_server::GraphRegistry;
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

    let sv_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("supervisor")
        .threaded()
        .core_threads(1)
        .build()
        .unwrap();

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .threaded()
        .core_threads(1)
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

    let g = GraphRegistry::new("./examples/registry_root");
    let server = toy_api_server::Server::new(g);

    sv_rt.spawn(async {
        let _ = sv.run().await;
    });
    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
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
