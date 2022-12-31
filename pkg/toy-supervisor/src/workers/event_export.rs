use crate::context::SupervisorContext;
use crate::exporters::EventExporter;
use toy_api_client::ApiClient;
use toy_core::metrics;

pub async fn event_export<C>(
    ctx: SupervisorContext<C>,
    exporter: Option<impl EventExporter>,
    interval: u64,
) where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!("event export...");
        let vec = metrics::context::events().drain().await;
        if let Some(ex) = exporter.as_ref() {
            if let Err(e) = ex.export(&ctx, vec).await {
                tracing::error!("{:?}", e);
            }
        }
        ctx.event_exported().await;

        toy_rt::sleep(interval).await;
    }
}
