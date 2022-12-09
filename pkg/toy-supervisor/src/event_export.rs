use crate::context::SupervisorContext;
use crate::exporters::Exporter;
use toy_api_client::ApiClient;

pub async fn event_export<C>(
    ctx: &SupervisorContext<C>,
    exporter: Option<impl Exporter>,
    interval: u64,
) where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!("event export...");
        let vec = ctx.events().drain().await;
        if let Some(ex) = exporter.as_ref() {
            if let Err(e) = ex.export(ctx, vec).await {
                tracing::error!("{:?}", e);
            }
            ctx.event_exported().await;
        }

        toy_rt::sleep(interval).await;
    }
}
