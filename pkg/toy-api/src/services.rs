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
