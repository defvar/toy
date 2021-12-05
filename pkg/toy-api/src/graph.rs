//! Model for graph api.

use crate::common::{ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use serde::{Deserialize, Serialize};
use toy_core::prelude::Value;
use toy_core::registry::PortType;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    name: String,
    services: Vec<GraphNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphNode {
    #[serde(rename = "type")]
    tp: String,
    uri: String,
    #[serde(default)]
    position: Position,
    port_type: Option<PortType>,
    config: Value,
    wires: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphList {
    graphs: Vec<Graph>,
    count: u32,
}

impl SelectionCandidate for Graph {
    fn candidate_map(&self) -> CandidateMap {
        CandidateMap::empty()
    }
}

impl GraphList {
    pub fn new(graphs: Vec<Graph>) -> Self {
        let count = graphs.len() as u32;
        Self { graphs, count }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl GraphListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for GraphListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> Selection {
        Selection::empty()
    }
}
