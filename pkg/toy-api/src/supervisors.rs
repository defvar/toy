//! Model for supervisor api.

use crate::common::{format, KVObject, ListObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::Candidates;
use crate::supervisors::SupervisorStatus::Ready;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use toy_core::data::Value;

pub type SupervisorName = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisorStatus {
    Ready,
    NoContact,
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
    addr: SocketAddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisorList {
    items: Vec<Supervisor>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupervisorBeatResponse {
    NotFound,
    Ok { last_beat_time: DateTime<Utc> },
}

impl Supervisor {
    pub fn new(
        name: String,
        start_time: DateTime<Utc>,
        labels: Vec<String>,
        addr: SocketAddr,
    ) -> Self {
        Self {
            name,
            start_time,
            labels,
            last_beat_time: None,
            status: Ready,
            last_transition_time: start_time,
            addr,
        }
    }

    pub fn name(&self) -> &SupervisorName {
        &self.name
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

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn with_last_beat_time(self, last_replied_on: DateTime<Utc>) -> Supervisor {
        let status = match self.status {
            SupervisorStatus::NoContact => SupervisorStatus::Ready,
            _ => self.status,
        };
        Self {
            last_beat_time: Some(last_replied_on),
            status,
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

    pub fn is_alive(&self) -> bool {
        match self.status {
            SupervisorStatus::Stop | SupervisorStatus::NoContact => false,
            _ => true,
        }
    }
}

impl KVObject for Supervisor {
    fn key(&self) -> &str {
        &self.name
    }
}

impl ListObject<Supervisor> for SupervisorList {
    fn items(&self) -> &[Supervisor] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
}

impl SelectionCandidate for Supervisor {
    fn candidate_fields() -> &'static [&'static str] {
        &["name", "start_time"]
    }

    fn candidates(&self) -> Candidates {
        Candidates::default()
            .with_candidate("name", Value::from(&self.name))
            .with_candidate("start_time", Value::from(&self.start_time))
    }
}

impl SupervisorList {
    pub fn new(items: Vec<Supervisor>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

//////////////////////////////////
// Option
//////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisorListOption {
    #[serde(flatten)]
    common: ListOption,
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
}
