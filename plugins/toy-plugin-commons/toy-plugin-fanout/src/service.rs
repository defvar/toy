use toy_core::prelude::{Frame, Outgoing, ServiceError, ServiceType};
use toy_core::service::ServiceContext;
use toy_core::task::TaskContext;
use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct BroadcastConfig {}

pub struct BroadcastContext {}

pub fn new_broadcast_context(
    _tp: ServiceType,
    _config: BroadcastConfig,
) -> Result<BroadcastContext, ServiceError> {
    Ok(BroadcastContext {})
}

pub async fn broadcast(
    _task_ctx: TaskContext,
    ctx: BroadcastContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<BroadcastContext>, ServiceError> {
    for p in tx.ports() {
        tx.send_ok_to(p, req.clone()).await?;
    }
    Ok(ServiceContext::Ready(ctx))
}
