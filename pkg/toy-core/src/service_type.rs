//! The type of service.
//!

use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Error, Formatter};

/// The type of service.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceType {
    full_name: String,
    name_space: String,
    service_name: String,
}

impl ServiceType {
    pub fn new<P: AsRef<str>>(name_space: P, service_name: P) -> Result<ServiceType, ConfigError> {
        let name_space = name_space.as_ref().to_string();
        let service_name = service_name.as_ref().to_string();
        if name_space.is_empty() {
            return Err(ConfigError::invalid_service_type(
                name_space,
                service_name,
                "name_space should not be empty.",
            ));
        }
        if service_name.is_empty() {
            return Err(ConfigError::invalid_service_type(
                name_space,
                service_name,
                "service_name should not be empty.",
            ));
        }
        Ok(ServiceType {
            full_name: format!("{}.{}", name_space, service_name),
            name_space,
            service_name,
        })
    }

    pub fn from_full_name<P: AsRef<str>>(full_name: P) -> Result<ServiceType, ConfigError> {
        let s = full_name.as_ref();
        if s.is_empty() {
            return Err(ConfigError::invalid_service_type(
                "",
                "",
                "serivce full name should not be empty.",
            ));
        }
        if !s.contains(".") {
            return Err(ConfigError::invalid_service_type(
                "",
                "",
                "service full name should contains \".\" (=name_space should not be empty).",
            ));
        }

        let full_name = s.to_string();
        let mut segments: Vec<&str> = s.split(".").collect();
        let service_name = segments.last().unwrap_or(&"").to_string();
        segments.pop();
        let name_space = segments.join(".").to_string();
        Ok(ServiceType {
            full_name,
            name_space,
            service_name,
        })
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn name_space(&self) -> &str {
        &self.name_space
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
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

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.full_name.to_string())
    }
}

impl Default for ServiceType {
    fn default() -> Self {
        ServiceType::new("builtin", "noop").unwrap()
    }
}
