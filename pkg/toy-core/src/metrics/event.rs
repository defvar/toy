use crate::prelude::TaskId;
use crate::Uri;
use serde::Serialize;
use std::collections::VecDeque;

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
    uri: Option<Uri>,
    event: String,
    timestamp: f64,
}

impl EventRecord {
    pub fn task_event(id: TaskId, event: impl Into<String>, timestamp: f64) -> EventRecord {
        Self {
            id,
            uri: None,
            event: event.into(),
            timestamp,
        }
    }

    pub fn service_event(
        id: TaskId,
        uri: &Uri,
        event: impl Into<String>,
        timestamp: f64,
    ) -> EventRecord {
        Self {
            id,
            uri: Some(uri.clone()),
            event: event.into(),
            timestamp,
        }
    }
}
