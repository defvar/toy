use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Registry, ServiceSchema};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};

/// An application that collects plugins.
#[derive(Clone)]
pub struct App<P> {
    plugin: P,
    schemas: Vec<ServiceSchema>,
}

impl<P> App<P>
where
    P: Registry,
{
    pub fn new(plugin: P) -> App<P> {
        let schemas = plugin.schemas().clone();
        App { plugin, schemas }
    }
}

impl<P> Registry for App<P>
where
    P: Registry,
{
    fn service_types(&self) -> Vec<ServiceType> {
        self.schemas
            .iter()
            .map(|x| x.service_type.clone())
            .collect()
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        self.schemas.clone()
    }

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> Result<(), ServiceError>
    where
        T: ServiceExecutor<Request = Frame, Error = ServiceError, InitError = ServiceError>,
    {
        match self.plugin.delegate(tp, uri, executor) {
            Ok(()) => Ok(()),
            Err(_) => Err(ServiceError::service_not_found(tp)),
        }
    }
}

impl<P> Debug for App<P>
where
    P: Registry,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App {{ services:[{:?}] }}", self.service_types())
    }
}
