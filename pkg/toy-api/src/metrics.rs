//! Model for metrics api.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    supervisor: String,
    timestamp: DateTime<Utc>,
    items: Vec<MetricsEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricsEntry {
    Counter(Counter),
    Gauge(Gauge),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Counter {
    name: String,
    value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gauge {
    name: String,
    value: f64,
}

impl Metrics {
    pub fn with(
        supervisor: impl Into<String>,
        timestamp: DateTime<Utc>,
        items: Vec<MetricsEntry>,
    ) -> Self {
        Self {
            supervisor: supervisor.into(),
            timestamp,
            items,
        }
    }

    pub fn supervisor(&self) -> &str {
        &self.supervisor
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn items(&self) -> &[MetricsEntry] {
        &self.items
    }
}

impl MetricsEntry {
    pub fn counter(name: impl Into<String>, v: u64) -> Self {
        MetricsEntry::Counter(Counter::with(name, v))
    }

    pub fn gauge(name: impl Into<String>, v: f64) -> Self {
        MetricsEntry::Gauge(Gauge::with(name, v))
    }
}

impl Counter {
    pub fn with(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Gauge {
    pub fn with(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
