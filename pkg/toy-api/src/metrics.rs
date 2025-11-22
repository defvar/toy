//! Model for metrics api.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    measurement: String,
    supervisor: String,
    timestamp: DateTime<Utc>,
    tags: Vec<MetricsTag>,
    items: Vec<MetricsEntry>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetricsTag {
    key: String,
    value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricsEntry {
    Counter(Counter),
    Gauge(Gauge),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Counter {
    name: String,
    value: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gauge {
    name: String,
    value: f64,
}

impl Metrics {
    pub fn with(
        measurement: impl Into<String>,
        supervisor: impl Into<String>,
        timestamp: DateTime<Utc>,
        tags: Vec<MetricsTag>,
        items: Vec<MetricsEntry>,
    ) -> Self {
        Self {
            measurement: measurement.into(),
            supervisor: supervisor.into(),
            timestamp,
            tags,
            items,
        }
    }

    pub fn measurement(&self) -> &str {
        &self.measurement
    }

    pub fn supervisor(&self) -> &str {
        &self.supervisor
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn tags(&self) -> &[MetricsTag] {
        &self.tags
    }

    pub fn items(&self) -> &[MetricsEntry] {
        &self.items
    }
}

impl MetricsTag {
    pub fn with(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for MetricsTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Debug for MetricsTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.key, self.value).as_str())
    }
}

impl MetricsEntry {
    pub fn counter(name: impl Into<String>, v: u64) -> Self {
        MetricsEntry::Counter(Counter::with(name, v))
    }

    pub fn gauge(name: impl Into<String>, v: f64) -> Self {
        MetricsEntry::Gauge(Gauge::with(name, v))
    }

    pub fn name(&self) -> &str {
        match self {
            MetricsEntry::Counter(_) => self.name(),
            MetricsEntry::Gauge(_) => self.name(),
        }
    }
}

impl Display for MetricsEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
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

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.name, self.value).as_str())
    }
}

impl Debug for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.name, self.value).as_str())
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

impl Display for Gauge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.name, self.value).as_str())
    }
}

impl Debug for Gauge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.name, self.value).as_str())
    }
}
