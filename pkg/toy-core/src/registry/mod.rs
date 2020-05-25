use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::ServiceExecutor;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;

mod app;
mod plugin;

pub use app::App;
pub use plugin::Plugin;

pub fn plugin<F>(name_space: &str, service_name: &str, callback: F) -> Plugin<NoopEntry, F> {
    Plugin::<NoopEntry, F>::new(name_space, service_name, callback)
}

pub fn app<P>(plugin: P) -> App<NoopEntry, P>
where
    P: Registry,
{
    App::<NoopEntry, P>::new(plugin)
}

pub trait PluginRegistry:
    Registry + Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>
{
}

pub trait Registry {
    fn service_types(&self) -> &Vec<ServiceType>;
}

pub trait Delegator {
    type Request;
    type Error: Error;
    type InitError: Error;

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >;
}

#[derive(Debug, Clone)]
pub struct NoopEntry;

impl Registry for NoopEntry {
    fn service_types(&self) -> &Vec<ServiceType> {
        unreachable!()
    }
}

impl Delegator for NoopEntry {
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn delegate<T>(
        &self,
        _tp: &ServiceType,
        _uri: &Uri,
        _executor: &mut T,
    ) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >,
    {
        Ok(())
    }
}
