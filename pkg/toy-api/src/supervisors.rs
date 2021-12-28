//! Model for supervisor api.

use crate::common::{format, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use crate::supervisors::SupervisorStatus::Ready;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type SupervisorName = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisorStatus {
    Ready,
    Running,
    Stop,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supervisor {
    name: SupervisorName,
    #[serde(with = "format::rfc3399")]
    start_time: DateTime<Utc>,
    labels: Vec<String>,
    #[serde(with = "format::rfc3399_option")]
    last_beat_time: Option<DateTime<Utc>>,
    status: SupervisorStatus,
    #[serde(with = "format::rfc3399")]
    last_transition_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisorList {
    items: Vec<Supervisor>,
    count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisorListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl Supervisor {
    pub fn new(name: String, start_time: DateTime<Utc>, labels: Vec<String>) -> Self {
        Self {
            name,
            start_time,
            labels,
            last_beat_time: None,
            status: Ready,
            last_transition_time: start_time,
        }
    }

    pub fn last_beat_time(&self) -> Option<&DateTime<Utc>> {
        self.last_beat_time.as_ref()
    }

    pub fn status(&self) -> SupervisorStatus {
        self.status
    }

    pub fn last_transition_time(&self) -> &DateTime<Utc> {
        &self.last_transition_time
    }

    pub fn with_last_beat_time(self, last_replied_on: DateTime<Utc>) -> Supervisor {
        Self {
            last_beat_time: Some(last_replied_on),
            ..self
        }
    }

    pub fn with_status(self, v: SupervisorStatus) -> Supervisor {
        Self { status: v, ..self }
    }

    pub fn with_last_transition_time(self, v: DateTime<Utc>) -> Supervisor {
        Self {
            last_transition_time: v,
            ..self
        }
    }
}

impl SelectionCandidate for Supervisor {
    fn candidate_map(&self) -> CandidateMap {
        CandidateMap::empty()
    }
}

impl SupervisorList {
    pub fn new(items: Vec<Supervisor>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

impl SupervisorListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for SupervisorListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> Selection {
        Selection::empty()
    }
}
