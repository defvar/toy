use std::fmt::{Debug, Error, Formatter};
use toy_pack::{Pack, Unpack};

#[derive(Clone, PartialEq, Eq, Hash, Pack, Unpack)]
pub struct ServiceType {
    full_name: String,
    name_space: String,
    service_name: String,
}

impl ServiceType {
    pub fn new<P: AsRef<str>>(name_space: P, service_name: P) -> ServiceType {
        let name_space = name_space.as_ref().to_string();
        let service_name = service_name.as_ref().to_string();
        assert!(!name_space.is_empty(), "name_space should not be empty");
        assert!(!service_name.is_empty(), "service_name should not be empty");
        ServiceType {
            full_name: format!("{}.{}", name_space, service_name),
            name_space,
            service_name,
        }
    }
}

impl<T> From<T> for ServiceType
where
    T: AsRef<str>,
{
    fn from(v: T) -> Self {
        let s = v.as_ref();
        assert!(!s.is_empty(), "serivce full name should not be empty");
        assert!(
            s.contains("."),
            "service full name should contains \".\" (=name_space should not be empty)"
        );

        let full_name = s.to_string();
        let mut segments: Vec<&str> = s.split(".").collect();
        let service_name = segments.last().unwrap_or(&"").to_string();
        segments.pop();
        let name_space = segments.join(".").to_string();
        ServiceType {
            full_name,
            name_space,
            service_name,
        }
    }
}

impl From<&ServiceType> for ServiceType {
    fn from(v: &ServiceType) -> Self {
        ServiceType {
            full_name: v.full_name.to_string(),
            name_space: v.name_space.to_string(),
            service_name: v.service_name.to_string(),
        }
    }
}

impl Debug for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.full_name.to_string())
    }
}
