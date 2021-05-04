use crate::common::Format;
use toy_core::data::schema::JsonSchema;
use toy_core::prelude::{PortType, ServiceType};
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ServiceSpecList {
    services: Vec<ServiceSpec>,
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
    pub fn new(services: Vec<ServiceSpec>) -> Self {
        let count = services.len() as u32;
        Self { services, count }
    }
}

impl Default for ServiceSpecList {
    fn default() -> Self {
        ServiceSpecList::new(Vec::new())
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct FindOption {
    format: Option<Format>,
}

impl FindOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct PutOption {
    format: Option<Format>,
}

impl PutOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct DeleteOption {
    format: Option<Format>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}
