use crate::metrics::kind::MetricsKind;
use crate::metrics::{Counter, Gauge};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MetricsRegistry {
    counters: RwLock<HashMap<MetricsKind, Counter>>,
    gauges: RwLock<HashMap<MetricsKind, Gauge>>,
}

impl MetricsRegistry {
    pub fn new() -> MetricsRegistry {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_counters(&self) -> HashMap<MetricsKind, Counter> {
        let mut counters = HashMap::new();
        let r = self.counters.read().await;
        for (k, v) in r.iter() {
            counters.insert(k.clone(), v.clone());
        }
        counters
    }

    pub async fn get_gauges(&self) -> HashMap<MetricsKind, Gauge> {
        let mut gauges = HashMap::new();
        let r = self.gauges.read().await;
        for (k, v) in r.iter() {
            gauges.insert(k.clone(), v.clone());
        }
        gauges
    }

    pub async fn counter<F>(&self, k: &MetricsKind, f: F)
    where
        F: FnOnce(&Counter) -> (),
    {
        let r = self.counters.read().await;
        if let Some(counter) = r.get(k) {
            f(counter)
        } else {
            drop(r);
            let mut w = self.counters.write().await;
            let v = w.entry(k.clone()).or_insert_with(|| Counter::from(0u64));
            f(v)
        }
    }

    pub async fn gauge<F>(&self, k: &MetricsKind, f: F)
    where
        F: FnOnce(&Gauge) -> (),
    {
        let r = self.gauges.read().await;
        if let Some(gauge) = r.get(k) {
            f(gauge)
        } else {
            drop(r);
            let mut w = self.gauges.write().await;
            let v = w.entry(k.clone()).or_insert_with(|| Gauge::from(0f64));
            f(v)
        }
    }

    pub async fn clear(&self) {
        self.counters.write().await.clear();
        self.gauges.write().await.clear();
    }
}
