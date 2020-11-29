use tokio::time::Duration;
use toy_core::prelude::{Frame, Outgoing, ServiceError, ServiceType};
use toy_core::service::ServiceContext;
use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct TickConfig {
    interval_millis: u64,
}

pub struct TickContext {
    count: u64,
    config: TickConfig,
}

pub fn new_tick_context(_tp: ServiceType, config: TickConfig) -> Result<TickContext, ServiceError> {
    Ok(TickContext { count: 0, config })
}

pub async fn tick(
    mut ctx: TickContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<TickContext>, ServiceError> {
    tokio::time::sleep(Duration::from_millis(ctx.config.interval_millis)).await;
    ctx.count += 1;
    tx.send_ok(Frame::from(ctx.count)).await?;
    Ok(ServiceContext::Next(ctx))
}
