use std::fmt::{Debug, Error, Formatter};
use toy_pack::{Pack, Unpack};

#[derive(Clone, PartialEq, Eq, Hash, Pack, Unpack)]
pub struct ServiceType {
    id: String,
}

impl From<&str> for ServiceType {
    fn from(v: &str) -> Self {
        ServiceType { id: v.to_string() }
    }
}

impl From<String> for ServiceType {
    fn from(v: String) -> Self {
        ServiceType { id: v }
    }
}

impl From<&String> for ServiceType {
    fn from(v: &String) -> Self {
        ServiceType { id: v.to_string() }
    }
}

impl From<&ServiceType> for ServiceType {
    fn from(v: &ServiceType) -> Self {
        ServiceType {
            id: v.id.to_string(),
        }
    }
}

impl Debug for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.id.to_string())
    }
}
