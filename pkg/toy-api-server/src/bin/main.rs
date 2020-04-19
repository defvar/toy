use std::sync::Arc;
use tokio::sync::Mutex;
use toy_api_server::{graphs, services};
use toy_core::prelude::*;
use toy_core::registry::Registry;
use toy_pack::UnPack;
use warp::Filter;

#[tokio::main]
async fn main() {
    let g = Arc::new(Mutex::new(Vec::new()));
    let regi = Registry::new("hello", factory!(hello, ServiceContextConfig, new_context))
        .service("hello2", factory!(hello, ServiceContextConfig, new_context));
    let regi = Arc::new(regi);

    let api = graphs(g).or(services(regi));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
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
