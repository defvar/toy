use crate::metrics::event_id::EventId;
use crate::metrics::kind::MetricsEventKind;
use crate::prelude::TaskId;
use crate::{ServiceType, Uri};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub struct MetricsEvents {
    raw: VecDeque<EventRecord>,
}

impl MetricsEvents {
    pub fn new() -> MetricsEvents {
        Self {
            raw: VecDeque::new(),
        }
    }

    pub fn push(&mut self, r: EventRecord) {
        self.raw.push_back(r);
    }

    pub fn extend<I: IntoIterator<Item = EventRecord>>(&mut self, iter: I) {
        self.raw.extend(iter);
    }

    pub fn records(&self) -> Vec<EventRecord> {
        self.raw.iter().cloned().collect()
    }
}

/// Structure of event information that has occurred.
/// "Start of Task", "Start of Service", etc.
#[derive(Debug, Clone, Serialize)]
pub struct EventRecord {
    event_id: EventId,
    task_id: TaskId,
    task_name: String,
    service_type: ServiceType,
    uri: Uri,
    event: MetricsEventKind,
    timestamp: DateTime<Utc>,
}

impl EventRecord {
    pub fn with_task(
        task_id: TaskId,
        task_name: impl Into<String>,
        uri: impl Into<Uri>,
        event: MetricsEventKind,
        timestamp: DateTime<Utc>,
    ) -> EventRecord {
        Self {
            event_id: EventId::new(),
            task_id,
            task_name: task_name.into(),
            service_type: ServiceType::noop(),
            uri: uri.into(),
            event,
            timestamp,
        }
    }

    pub fn with_service(
        task_id: TaskId,
        task_name: impl Into<String>,
        service_type: impl Into<ServiceType>,
        uri: impl Into<Uri>,
        event: MetricsEventKind,
        timestamp: DateTime<Utc>,
    ) -> EventRecord {
        Self {
            event_id: EventId::new(),
            task_id,
            task_name: task_name.into(),
            service_type: service_type.into(),
            uri: uri.into(),
            event,
            timestamp,
        }
    }

    pub fn event_id(&self) -> EventId {
        self.event_id
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn task_name(&self) -> &str {
        &self.task_name
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn event(&self) -> &MetricsEventKind {
        &self.event
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
