use crate::context::SupervisorContext;
use crate::exporters::common::MetricsExporter;
use crate::exporters::EventExporter;
use crate::SupervisorError;
use chrono::Utc;
use toy_api::common::PostOption;
use toy_api::metrics::{Metrics, MetricsEntry};
use toy_api::task::TaskEvent;
use toy_api_client::client::{MetricsClient, TaskClient};
use toy_api_client::ApiClient;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::EventRecord;

pub struct ToyExporter<T> {
    client: T,
}

impl<T> ToyExporter<T>
where
    T: ApiClient + Clone + Send + Sync + 'static,
{
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl<T> EventExporter for ToyExporter<T>
where
    T: ApiClient + Clone + Send + Sync + 'static,
{
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        events: Vec<EventRecord>,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let mut body = vec![];
        for item in &events {
            body.push(TaskEvent::new(
                item.id(),
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

        self.client
            .task()
            .post_event(body, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}

#[async_trait::async_trait]
impl<T> MetricsExporter for ToyExporter<T>
where
    T: ApiClient + Clone,
{
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        metrics: &MetricsRegistry,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let now = Utc::now();
        let counters = metrics.get_counters().await;
        let gauges = metrics.get_gauges().await;

        let mut counters = counters
            .iter()
            .map(|(k, v)| MetricsEntry::counter(k.as_kind_text(), v.get().unwrap_or(0)))
            .collect::<Vec<_>>();

        let mut gauges = gauges
            .iter()
            .map(|(k, v)| MetricsEntry::gauge(k.as_kind_text(), v.get().unwrap_or(0f64)))
            .collect::<Vec<_>>();

        counters.append(&mut gauges);

        if counters.len() <= 0 {
            return Ok(());
        }

        let v = Metrics::with(ctx.name(), now, counters);

        self.client
            .metrics()
            .post(v, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}
