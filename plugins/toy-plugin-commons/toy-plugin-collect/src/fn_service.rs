use crate::config::FirstConfig;
use toy_core::prelude::{Frame, Outgoing, ServiceContext, ServiceError, ServiceType, TaskContext};

pub struct FirstContext {}

pub fn new_first_context(
    _tp: ServiceType,
    _config: FirstConfig,
) -> Result<FirstContext, ServiceError> {
    Ok(FirstContext {})
}

pub async fn first(
    _task_ctx: TaskContext,
    ctx: FirstContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<FirstContext>, ServiceError> {
    tracing::debug!(send =?req);
    tx.send_ok(req).await?;
    Ok(ServiceContext::Complete(ctx))
}
