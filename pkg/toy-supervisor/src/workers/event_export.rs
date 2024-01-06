use crate::context::SupervisorContext;
use crate::exporters::EventExporter;
use toy_api_client::ApiClient;
use toy_core::metrics;

pub async fn start_event_exporter<C>(
    ctx: SupervisorContext<C>,
    exporter: impl EventExporter,
    interval: u64,
) where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!("event export...");
        let vec = metrics::context::events().drain().await;

        if let Err(e) = exporter.export(&ctx, &vec).await {
            tracing::error!("{:?}", e);
            //recover
            metrics::context::events().extend(vec).await;
        }

        ctx.event_exported().await;

        toy_rt::sleep(interval).await;
    }
}
