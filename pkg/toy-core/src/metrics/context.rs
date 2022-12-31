use crate::metrics::registry::event::EventRegistry;
use crate::metrics::registry::metrics::MetricsRegistry;
use crate::metrics::MetricsEvents;
use crate::task::TaskId;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::Mutex;

static METRICS_REGISTRY: OnceCell<MetricsRegistry> = OnceCell::new();

static EVENT_REGISTRY: OnceCell<EventRegistry> = OnceCell::new();

pub fn metrics() -> &'static MetricsRegistry {
    METRICS_REGISTRY.get_or_init(|| MetricsRegistry::new())
}

pub fn events() -> &'static EventRegistry {
    EVENT_REGISTRY.get_or_init(|| EventRegistry::new())
}

pub async fn events_by_task_id(id: TaskId) -> Arc<Mutex<MetricsEvents>> {
    events().get_or_create(id).await
}
