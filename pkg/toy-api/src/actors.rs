//! Model for actor api.

use crate::actors::ActorStatus::Ready;
use crate::common::{format, KVObject, ListObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::Candidates;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub type ActorName = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorStatus {
    Ready,
    NoContact,
    Stop,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    name: ActorName,
    #[serde(with = "format::rfc3399")]
    start_time: DateTime<Utc>,
    labels: Vec<String>,
    #[serde(with = "format::rfc3399_option")]
    last_beat_time: Option<DateTime<Utc>>,
    status: ActorStatus,
    #[serde(with = "format::rfc3399")]
    last_transition_time: DateTime<Utc>,
    addr: SocketAddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorList {
    items: Vec<Actor>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorBeatResponse {
    NotFound,
    Ok { last_beat_time: DateTime<Utc> },
}

impl Actor {
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

    pub fn name(&self) -> &ActorName {
        &self.name
    }

    pub fn last_beat_time(&self) -> Option<&DateTime<Utc>> {
        self.last_beat_time.as_ref()
    }

    pub fn status(&self) -> ActorStatus {
        self.status
    }

    pub fn last_transition_time(&self) -> &DateTime<Utc> {
        &self.last_transition_time
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn with_last_beat_time(self, last_replied_on: DateTime<Utc>) -> Actor {
        let status = match self.status {
            ActorStatus::NoContact => ActorStatus::Ready,
            _ => self.status,
        };
        Self {
            last_beat_time: Some(last_replied_on),
            status,
            ..self
        }
    }

    pub fn with_status(self, v: ActorStatus) -> Actor {
        Self { status: v, ..self }
    }

    pub fn with_last_transition_time(self, v: DateTime<Utc>) -> Actor {
        Self {
            last_transition_time: v,
            ..self
        }
    }

    pub fn is_alive(&self) -> bool {
        match self.status {
            ActorStatus::Stop | ActorStatus::NoContact => false,
            _ => true,
        }
    }
}

impl KVObject for Actor {
    fn key(&self) -> &str {
        &self.name
    }
}

impl ListObject<Actor> for ActorList {
    fn items(&self) -> &[Actor] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
}

impl SelectionCandidate for Actor {
    fn candidate_fields() -> &'static [&'static str] {
        &["name", "start_time"]
    }

    fn candidates(&self) -> Candidates {
        Candidates::default()
            .with_candidate("name", &self.name)
            .with_candidate("start_time", &self.start_time)
    }
}

impl ActorList {
    pub fn new(items: Vec<Actor>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

//////////////////////////////////
// Option
//////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl ActorListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for ActorListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }
}
