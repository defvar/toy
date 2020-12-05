use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, NoopEntry, PluginRegistry, Registry, ServiceSchema};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct App<O, P> {
    inner: Inner<O, P>,
    schemas: Vec<ServiceSchema>,
}

#[derive(Clone)]
struct Inner<O, P> {
    other: Option<O>,
    plugin: P,
}

impl<O, P> App<O, P>
where
    O: Registry,
    P: Registry,
{
    pub fn new(plugin: P) -> App<NoopEntry, P> {
        let schemas = plugin.schemas().clone();
        App {
            inner: Inner {
                other: Option::<NoopEntry>::None,
                plugin,
            },
            schemas,
        }
    }

    pub fn with<P2>(self, plugin: P2) -> App<Self, P2>
    where
        Self: Sized,
        P2: Registry,
    {
        let mut schemas = self.schemas.clone();
        schemas.extend_from_slice(&plugin.schemas());
        App {
            inner: Inner {
                other: Some(self),
                plugin,
            },
            schemas,
        }
    }
}

impl<O, P> Delegator for App<O, P>
where
    O: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
    P: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
{
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >,
    {
        match &self.inner.other {
            Some(other) => match other.delegate(tp, uri, executor) {
                Ok(()) => Ok(()),
                Err(_) => match self.inner.plugin.delegate(tp, uri, executor) {
                    Ok(()) => Ok(()),
                    Err(_) => Err(ServiceError::service_not_found(tp)),
                },
            },
            None => match self.inner.plugin.delegate(tp, uri, executor) {
                Ok(()) => Ok(()),
                Err(_) => Err(ServiceError::service_not_found(tp)),
            },
        }
    }
}

impl<O, P> PluginRegistry for App<O, P>
where
    O: PluginRegistry,
    P: PluginRegistry,
{
}

impl<O, P> Registry for App<O, P> {
    fn service_types(&self) -> Vec<ServiceType> {
        self.schemas
            .iter()
            .map(|x| x.service_type.clone())
            .collect()
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        self.schemas.clone()
    }
}

impl<O, P> Debug for App<O, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App {{ services:[{:?}] }}", self.service_types())
    }
}
