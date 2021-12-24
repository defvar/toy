//! Model for supervisor api.

use crate::common::{format, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supervisor {
    name: String,
    #[serde(with = "format::rfc3399")]
    started_on: DateTime<Utc>,
    labels: Vec<String>,
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
    pub fn new(name: String, started_on: DateTime<Utc>, labels: Vec<String>) -> Self {
        Self {
            name,
            started_on,
            labels,
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
