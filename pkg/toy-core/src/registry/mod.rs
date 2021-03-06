//! Registry for services.
//! Register a service that can be defined as a Graph.
//!

use crate::data::schema::visitors::JsonSchemaVisitor;
use crate::data::schema::JsonSchema;
use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::ServiceExecutor;
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use toy_pack::schema::{to_schema, Schema};
use toy_pack::{Pack, Unpack};

mod app;
mod layered;
mod plugin;
mod port_type;

pub use app::App;
pub use layered::Layered;
pub use plugin::Plugin;
pub use port_type::PortType;

/// Create plugin.
pub fn plugin<F, R>(
    name_space: &str,
    service_name: &str,
    port_type: PortType,
    callback: F,
) -> Plugin<F>
where
    F: Fn() -> R + Clone,
    R: ServiceFactory,
    R::Config: Schema,
{
    Plugin::<F>::new(name_space, service_name, port_type, callback)
}

/// Create app.
pub fn app<P>(plugin: P) -> App<NoopEntry, P>
where
    P: Registry,
{
    App::<NoopEntry, P>::new(plugin)
}

pub trait PluginRegistry:
    Registry + Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError> + Clone
{
}

pub trait Registry {
    fn service_types(&self) -> Vec<ServiceType>;

    fn schemas(&self) -> Vec<ServiceSchema>;
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

/// ServiceSchema (json schema format) for front-end api.
#[derive(Debug, Clone, Pack, Unpack)]
pub struct ServiceSchema {
    service_type: ServiceType,
    port_type: PortType,
    schema: Option<JsonSchema>,
}

impl ServiceSchema {
    pub fn new<T>(name_space: &str, service_name: &str, port_type: PortType) -> Self
    where
        T: Schema,
    {
        let tp = ServiceType::new(name_space, service_name).unwrap();
        let schema = to_schema::<T, JsonSchemaVisitor>(service_name, JsonSchemaVisitor)
            .map_err(|e| tracing::error!("an error occured; {:?}", e))
            .ok();
        Self {
            service_type: tp,
            port_type,
            schema,
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }

    pub fn port_type(&self) -> &PortType {
        &self.port_type
    }

    pub fn schema(&self) -> Option<&JsonSchema> {
        self.schema.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct NoopEntry;

impl PluginRegistry for NoopEntry {}

impl Registry for NoopEntry {
    fn service_types(&self) -> Vec<ServiceType> {
        unreachable!()
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
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
