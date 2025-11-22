use crate::context::SupervisorContext;
use crate::exporters::util::{supervisor_metrics, worker_metrics};
use crate::exporters::{EventExporter, MetricsExporter};
use crate::SupervisorError;
use chrono::Utc;
use toy_api_client::ApiClient;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::EventRecord;
use tracing::instrument;

pub struct TracingExporter;

#[async_trait::async_trait]
impl EventExporter for TracingExporter {
    #[instrument(name = "event", level = "debug", skip_all)]
    async fn export<C>(
        &self,
        _ctx: &SupervisorContext<C>,
        events: &[EventRecord],
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        for item in events {
            tracing::info!(?item);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl MetricsExporter for TracingExporter {
    #[instrument(name = "metrics", level = "debug", skip_all)]
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        metrics: &MetricsRegistry,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let now = Utc::now();
        let mut candidates = vec![];
        candidates.push(supervisor_metrics(now, ctx.name(), metrics).await);
        candidates.extend_from_slice(&worker_metrics(now, ctx.name()).await);
        for item in candidates {
            tracing::info!(
                measurement = %item.measurement(),
                supervisor = %item.supervisor(),
                timestamp = %item.timestamp(),
                tags = ?item.tags(),
                items = ?item.items(),
            );
        }
        Ok(())
    }
}
