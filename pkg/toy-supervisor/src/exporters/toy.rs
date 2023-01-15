use crate::context::SupervisorContext;
use crate::exporters::common::MetricsExporter;
use crate::exporters::EventExporter;
use crate::SupervisorError;
use chrono::Utc;
use toy_api::common::PostOption;
use toy_api::metrics::{Metrics, MetricsEntry, MetricsTag};
use toy_api::task::TaskEvent;
use toy_api_client::client::{MetricsClient, TaskClient};
use toy_api_client::ApiClient;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::{Counter, EventRecord, Gauge};

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
        let mut candidates = vec![];

        candidates.push({
            let counters = metrics.get_counters().await;
            let gauges = metrics.get_gauges().await;
            let items = to_metrics_entries(
                counters.iter().map(|(k, v)| (k.as_kind_text(), v.clone())),
                gauges.iter().map(|(k, v)| (k.as_kind_text(), v.clone())),
            );
            let sv = Metrics::with("supervisor", ctx.name(), now, Vec::with_capacity(0), items);
            sv
        });

        candidates.extend_from_slice(&{
            let mut candidates = vec![];
            let runtime_metrics = toy_rt::metrics();
            let counters = runtime_metrics.get_counters();
            let gauges = runtime_metrics.get_gauges();
            let items = to_metrics_entries(counters.into_iter(), gauges.into_iter());
            let runtime = Metrics::with("runtime", ctx.name(), now, Vec::with_capacity(0), items);
            candidates.push(runtime);

            for w in runtime_metrics.workers() {
                let w_counters = w.get_counters();
                let w_gauges = w.get_gauges();
                let items = to_metrics_entries(w_counters.into_iter(), w_gauges.into_iter());

                let worker = Metrics::with(
                    "runtime_worker",
                    ctx.name(),
                    now,
                    vec![MetricsTag::with("worker", w.worker().to_string())],
                    items,
                );
                candidates.push(worker);
            }
            candidates
        });

        self.client
            .metrics()
            .post(candidates, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}

fn to_metrics_entries<'a>(
    counters: impl Iterator<Item = (impl Into<String>, Counter)>,
    gauges: impl Iterator<Item = (impl Into<String>, Gauge)>,
) -> Vec<MetricsEntry> {
    let mut counters = counters
        .map(|(k, v)| MetricsEntry::counter(k, v.get().unwrap_or(0)))
        .collect::<Vec<_>>();

    let mut gauges = gauges
        .map(|(k, v)| MetricsEntry::gauge(k, v.get().unwrap_or(0f64)))
        .collect::<Vec<_>>();

    counters.append(&mut gauges);
    counters
}
