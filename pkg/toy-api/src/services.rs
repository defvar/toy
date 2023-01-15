//! Model for services api.

use crate::common::{KVObject, ListObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::Candidates;
use serde::{Deserialize, Serialize};
use toy_core::data::schema::JsonSchema;
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

impl KVObject for ServiceSpec {
    fn key(&self) -> &str {
        self.service_type.full_name()
    }
}

impl ListObject<ServiceSpec> for ServiceSpecList {
    fn items(&self) -> &[ServiceSpec] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
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
    fn candidate_fields() -> &'static [&'static str] {
        &["name_space", "port_type"]
    }

    fn candidates(&self) -> Candidates {
        let p = match self.port_type {
            PortType::Source(_) => "Source",
            PortType::Flow(_, _) => "Flow",
            PortType::Sink(_) => "Sink",
        };
        Candidates::default()
            .with_candidate("name_space", self.service_type.name_space())
            .with_candidate("port_type", p)
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

//////////////////////////////////
// Option
//////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceSpecListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl ServiceSpecListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for ServiceSpecListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }
}
