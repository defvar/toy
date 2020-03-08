use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl Debug for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.id.to_string())
    }
}
