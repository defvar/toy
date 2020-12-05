use tokio::time::Duration;
use toy_core::prelude::{Frame, Outgoing, ServiceError, ServiceType};
use toy_core::service::ServiceContext;
use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct TickConfig {
    interval_millis: u64,
    start: u64,
    end: Option<u64>,
}

pub struct TickContext {
    count: u64,
    config: TickConfig,
}

pub fn new_tick_context(_tp: ServiceType, config: TickConfig) -> Result<TickContext, ServiceError> {
    Ok(TickContext {
        count: config.start,
        config,
    })
}

pub async fn tick(
    mut ctx: TickContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<TickContext>, ServiceError> {
    tokio::time::sleep(Duration::from_millis(ctx.config.interval_millis)).await;
    tx.send_ok(Frame::from(ctx.count)).await?;
    match ctx.config.end {
        Some(end) if end <= ctx.count => Ok(ServiceContext::Complete(ctx)),
        _ => {
            ctx.count += 1;
            Ok(ServiceContext::Next(ctx))
        }
    }
}
