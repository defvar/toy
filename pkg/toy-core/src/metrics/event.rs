use crate::metrics::{Counter, Gauge};
use crate::prelude::TaskId;
use crate::{ServiceType, Uri};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub enum MetricsEvent {
    StartTask,
    FinishTask,
    StartService,
    FinishService,
    SendRequest,
    ReceiveRequest,
    ReceiveStop,
    ReceiveUpstreamFinish,
    FinishUpstreamFinish,
    FinishUpstreamFinishAll,
    CustomEvent(String),
    CustomCounter(String, Counter),
    CustomGauge(String, Gauge),
}

impl MetricsEvent {
    pub fn as_event_text(&self) -> &str {
        match self {
            MetricsEvent::StartTask => "StartTask",
            MetricsEvent::FinishTask => "FinishTask",
            MetricsEvent::StartService => "StartService",
            MetricsEvent::FinishService => "FinishService",
            MetricsEvent::SendRequest => "SendRequest",
            MetricsEvent::ReceiveRequest => "ReceiveRequest",
            MetricsEvent::ReceiveStop => "ReceiveStop",
            MetricsEvent::ReceiveUpstreamFinish => "ReceiveUpstreamFinish",
            MetricsEvent::FinishUpstreamFinish => "FinishUpstreamFinish",
            MetricsEvent::FinishUpstreamFinishAll => "FinishUpstreamFinishAll",
            MetricsEvent::CustomEvent(e) => e,
            MetricsEvent::CustomCounter(e, _) => e,
            MetricsEvent::CustomGauge(e, _) => e,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Events {
    raw: VecDeque<EventRecord>,
}

impl Events {
    pub fn new() -> Events {
        Self {
            raw: VecDeque::new(),
        }
    }

    pub fn push(&mut self, r: EventRecord) {
        self.raw.push_back(r);
    }

    pub fn records(&self) -> Vec<EventRecord> {
        self.raw.iter().cloned().collect()
    }
}

/// Structure of event information that has occurred.
/// "Start of Task", "Start of Service", etc.
#[derive(Debug, Clone, Serialize)]
pub struct EventRecord {
    id: TaskId,
    name: String,
    service_type: ServiceType,
    uri: Uri,
    event: MetricsEvent,
    timestamp: DateTime<Utc>,
}

impl EventRecord {
    pub fn with_task(
        id: TaskId,
        name: impl Into<String>,
        uri: impl Into<Uri>,
        event: MetricsEvent,
        timestamp: DateTime<Utc>,
    ) -> EventRecord {
        Self {
            id,
            name: name.into(),
            service_type: ServiceType::noop(),
            uri: uri.into(),
            event,
            timestamp,
        }
    }

    pub fn with_service(
        id: TaskId,
        name: impl Into<String>,
        service_type: impl Into<ServiceType>,
        uri: impl Into<Uri>,
        event: MetricsEvent,
        timestamp: DateTime<Utc>,
    ) -> EventRecord {
        Self {
            id,
            name: name.into(),
            service_type: service_type.into(),
            uri: uri.into(),
            event,
            timestamp,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn event(&self) -> &MetricsEvent {
        &self.event
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
