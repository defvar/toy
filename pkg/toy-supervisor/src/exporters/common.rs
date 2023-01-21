use crate::context::SupervisorContext;
use crate::SupervisorError;
use toy_api_client::ApiClient;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::EventRecord;

#[async_trait::async_trait]
pub trait EventExporter: Sync + Send {
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        events: &[EventRecord],
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static;
}

#[async_trait::async_trait]
pub trait MetricsExporter: Sync + Send {
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        metrics: &MetricsRegistry,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static;
}

pub struct NoopExporter;

#[async_trait::async_trait]
impl EventExporter for NoopExporter {
    async fn export<C>(
        &self,
        _ctx: &SupervisorContext<C>,
        _events: &[EventRecord],
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        Ok(())
    }
}

#[async_trait::async_trait]
impl MetricsExporter for NoopExporter {
    async fn export<C>(
        &self,
        _ctx: &SupervisorContext<C>,
        _metrics: &MetricsRegistry,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        Ok(())
    }
}
