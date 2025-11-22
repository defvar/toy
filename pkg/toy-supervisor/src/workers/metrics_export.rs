use crate::context::SupervisorContext;
use crate::exporters::MetricsExporter;
use toy_api_client::ApiClient;
use toy_core::metrics;

pub async fn start_metrics_exporter<C>(
    ctx: SupervisorContext<C>,
    exporter: impl MetricsExporter,
    interval: u64,
) where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        if let Err(e) = exporter.export(&ctx, metrics::context::metrics()).await {
            tracing::error!("{}", e);
        }
        ctx.metrics_exported().await;
        toy_rt::sleep(interval).await;
    }
}
