use toy_core::data::schema::JsonSchema;
use toy_core::prelude::{PortType, ServiceType};
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ServiceSpecList {
    items: Vec<ServiceSpec>,
    count: u32,
}

#[derive(Debug, Clone, Pack, Unpack)]
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
