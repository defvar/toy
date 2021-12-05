use crate::common::{ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use serde::{Deserialize, Serialize};
use toy_core::data::schema::JsonSchema;
use toy_core::data::Value;
use toy_core::prelude::{PortType, ServiceType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceSpecList {
    items: Vec<ServiceSpec>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    service_type: ServiceType,
    port_type: PortType,
    schema: Option<JsonSchema>,
}

impl ServiceSpec {
    pub fn new(service_type: ServiceType, port_type: PortType, schema: Option<JsonSchema>) -> Self {
        Self {
            service_type,
            port_type,
            schema,
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }
}

impl SelectionCandidate for ServiceSpec {
    fn candidate_map(&self) -> CandidateMap {
        CandidateMap::default()
            .with_candidate("name_space", Value::from(self.service_type.name_space()))
    }
}

impl ServiceSpecList {
    pub fn new(items: Vec<ServiceSpec>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

impl Default for ServiceSpecList {
    fn default() -> Self {
        ServiceSpecList::new(Vec::new())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceSpecListOption {
    #[serde(flatten)]
    common: ListOption,
    name_space: Option<String>,
}

impl ServiceSpecListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
            name_space: None,
        }
    }

    pub fn name_space(&self) -> Option<&str> {
        self.name_space.as_ref().map(|x| x.as_str())
    }
}

impl ListOptionLike for ServiceSpecListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> Selection {
        Selection::default().contains_if_some("name_space", self.name_space())
    }
}
