//! Model for services api.

use crate::common::{KVObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::selector::Selector;
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

impl KVObject for ServiceSpec {
    fn key(&self) -> &str {
        self.service_type.full_name()
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

    fn candidate_map(&self) -> CandidateMap {
        let p = match self.port_type {
            PortType::Source(_) => "Source",
            PortType::Flow(_, _) => "Flow",
            PortType::Sink(_) => "Sink",
        };
        CandidateMap::default()
            .with_candidate("name_space", Value::from(self.service_type.name_space()))
            .with_candidate("port_type", Value::from(p))
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
    name_space: Option<String>,
    port_type: Option<String>,
}

impl ServiceSpecListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
            name_space: None,
            port_type: None,
        }
    }

    pub fn name_space(&self) -> Option<&str> {
        self.name_space.as_ref().map(|x| x.as_str())
    }

    pub fn port_type(&self) -> Option<&str> {
        self.port_type.as_ref().map(|x| x.as_str())
    }
}

impl ListOptionLike for ServiceSpecListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> &Selector {
        self.common.selection()
    }
}

#[cfg(test)]
mod tests {
    use crate::selection::Operator;
    use crate::services::ServiceSpecListSelectors;

    // #[test]
    // fn aaaaa() {
    //     let r = toy_pack_json::unpack(b"").unwrap();
    //     println!("{:?}", r);
    // }
}
