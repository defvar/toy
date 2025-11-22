use chrono::{DateTime, Utc};
use toy_api::metrics::{Metrics, MetricsEntry, MetricsTag};
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::{Counter, Gauge};

pub async fn supervisor_metrics(
    now: DateTime<Utc>,
    name: &str,
    metrics: &MetricsRegistry,
) -> Metrics {
    let counters = metrics.get_counters().await;
    let gauges = metrics.get_gauges().await;
    let items = to_metrics_entries(
        counters.iter().map(|(k, v)| (k.as_kind_text(), v.clone())),
        gauges.iter().map(|(k, v)| (k.as_kind_text(), v.clone())),
    );
    Metrics::with("supervisor", name, now, Vec::with_capacity(0), items)
}

pub async fn worker_metrics(now: DateTime<Utc>, name: &str) -> Vec<Metrics> {
    let mut candidates = vec![];
    let runtime_metrics = toy_rt::metrics();
    let counters = runtime_metrics.get_counters();
    let gauges = runtime_metrics.get_gauges();
    let items = to_metrics_entries(counters.into_iter(), gauges.into_iter());
    let runtime = Metrics::with("runtime", name, now, Vec::with_capacity(0), items);
    candidates.push(runtime);

    for w in runtime_metrics.workers() {
        let w_counters = w.get_counters();
        let w_gauges = w.get_gauges();
        let items = to_metrics_entries(w_counters.into_iter(), w_gauges.into_iter());

        let worker = Metrics::with(
            "runtime_worker",
            name,
            now,
            vec![MetricsTag::with("worker", w.worker().to_string())],
            items,
        );
        candidates.push(worker);
    }
    candidates
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
