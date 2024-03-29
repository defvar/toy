//! Model for graph api.

use crate::common::{KVObject, Label, ListObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::Candidates;
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
    #[serde(default)]
    disabled: bool,
    services: Vec<GraphNode>,
    #[serde(default = "Vec::new")]
    labels: Vec<Label>,
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
    items: Vec<Graph>,
    count: u32,
}

impl SelectionCandidate for Graph {
    fn candidate_fields() -> &'static [&'static str] {
        &[]
    }

    fn candidates(&self) -> Candidates {
        Candidates::empty()
    }
}

impl KVObject for Graph {
    fn key(&self) -> &str {
        &self.name
    }
}

impl ListObject<Graph> for GraphList {
    fn items(&self) -> &[Graph] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
}

impl Graph {
    pub fn new(
        name: impl Into<String>,
        disabled: bool,
        services: Vec<GraphNode>,
        labels: Vec<Label>,
    ) -> Self {
        Self {
            name: name.into(),
            disabled,
            services,
            labels,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn services(&self) -> &[GraphNode] {
        &self.services
    }

    pub fn labels(&self) -> &[Label] {
        &self.labels
    }
}

impl GraphNode {
    pub fn new(
        tp: impl Into<String>,
        uri: impl Into<String>,
        position: Position,
        port_type: Option<PortType>,
        config: Value,
        wires: Vec<String>,
    ) -> Self {
        Self {
            tp: tp.into(),
            uri: uri.into(),
            position,
            port_type,
            config,
            wires,
        }
    }
}

impl GraphList {
    pub fn new(items: Vec<Graph>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

//////////////////////////////////
// Option
//////////////////////////////////

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
}
