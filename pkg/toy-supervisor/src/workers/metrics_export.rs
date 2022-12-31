use crate::context::SupervisorContext;
use crate::exporters::MetricsExporter;
use toy_api_client::ApiClient;
use toy_core::metrics;

pub async fn metrics_export<C>(
    ctx: SupervisorContext<C>,
    exporter: Option<impl MetricsExporter>,
    interval: u64,
) where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!("metrics export...");
        if let Some(ex) = exporter.as_ref() {
            if let Err(e) = ex.export(&ctx, metrics::context::metrics()).await {
                tracing::error!("{}", e);
            }
            ctx.metrics_exported().await;
        }

        toy_rt::sleep(interval).await;
    }
}
