use crate::context::ActorContext;
use crate::exporters::common::MetricsExporter;
use crate::exporters::util::*;
use crate::exporters::EventExporter;
use crate::ActorError;
use chrono::Utc;
use toy_api::common::PostOption;
use toy_api::task::TaskEvent;
use toy_api_client::client::{MetricsClient, TaskClient};
use toy_api_client::ApiClient;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::EventRecord;
use tracing::instrument;

pub struct ToyExporter;

#[async_trait::async_trait]
impl EventExporter for ToyExporter {
    #[instrument(name = "event", level = "debug", skip_all)]
    async fn export<C>(
        &self,
        ctx: &ActorContext<C>,
        events: &[EventRecord],
    ) -> Result<(), ActorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let mut body = vec![];
        for item in events {
            body.push(TaskEvent::new(
                item.event_id(),
                item.task_id(),
                item.task_name(),
                item.service_type().clone(),
                item.uri().clone(),
                item.event().as_event_text(),
                ctx.name(),
                item.timestamp().clone(),
            ));
        }
        if body.is_empty() {
            return Ok(());
        }

        ctx.client()
            .unwrap()
            .task()
            .post_event(body, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}

#[async_trait::async_trait]
impl MetricsExporter for ToyExporter {
    #[instrument(name = "metrics", level = "debug", skip_all)]
    async fn export<C>(
        &self,
        ctx: &ActorContext<C>,
        metrics: &MetricsRegistry,
    ) -> Result<(), ActorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let now = Utc::now();
        let mut candidates = vec![];
        candidates.push(actor_metrics(now, ctx.name(), metrics).await);
        candidates.extend_from_slice(&worker_metrics(now, ctx.name()).await);

        ctx.client()
            .unwrap()
            .metrics()
            .post(candidates, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}
