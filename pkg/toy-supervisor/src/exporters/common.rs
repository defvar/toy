use crate::context::SupervisorContext;
use crate::SupervisorError;
use toy_api_client::ApiClient;
use toy_core::metrics::EventRecord;

#[async_trait::async_trait]
pub trait Exporter: Sync + Send {
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        events: Vec<EventRecord>,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static;
}

pub struct NoopExporter;

#[async_trait::async_trait]
impl Exporter for NoopExporter {
    async fn export<C>(
        &self,
        _ctx: &SupervisorContext<C>,
        _events: Vec<EventRecord>,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        Ok(())
    }
}
