use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, NoopEntry, Registry};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};

pub struct App<O, P> {
    other: Option<O>,
    plugin: P,
    tps: Vec<ServiceType>,
}

impl<O, P> App<O, P>
where
    O: Registry,
    P: Registry,
{
    pub fn new(plugin: P) -> App<NoopEntry, P> {
        let tps = plugin.service_types().clone();
        App {
            other: Option::<NoopEntry>::None,
            plugin,
            tps,
        }
    }

    pub fn plugin<P2>(self, plugin: P2) -> App<Self, P2>
    where
        Self: Sized,
        P2: Registry,
    {
        let mut tps = self.tps.clone();
        tps.extend_from_slice(plugin.service_types());
        App {
            other: Some(self),
            plugin,
            tps,
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
        match &self.other {
            Some(other) => match other.delegate(tp, uri, executor) {
                Ok(()) => Ok(()),
                Err(_) => match self.plugin.delegate(tp, uri, executor) {
                    Ok(()) => Ok(()),
                    Err(_) => Err(ServiceError::service_not_found(tp)),
                },
            },
            None => match self.plugin.delegate(tp, uri, executor) {
                Ok(()) => Ok(()),
                Err(_) => Err(ServiceError::service_not_found(tp)),
            },
        }
    }
}

impl<O, P> Registry for App<O, P> {
    fn service_types(&self) -> &Vec<ServiceType> {
        &self.tps
    }
}

impl<O, P> Debug for App<O, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SystemRegistry {{ services:[{:?}] }}", self.tps)
    }
}
