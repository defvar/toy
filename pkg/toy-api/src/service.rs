use toy_core::registry::ServiceSchema;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ServicesEntity {
    services: Vec<ServiceSchema>,
    count: u32,
}

impl ServicesEntity {
    pub fn new(services: Vec<ServiceSchema>) -> Self {
        let count = services.len() as u32;
        Self { services, count }
    }
}

impl Default for ServicesEntity {
    fn default() -> Self {
        ServicesEntity::new(Vec::new())
    }
}
