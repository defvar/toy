//! Registry for services.
//! Register a service that can be defined as a Graph.
//!

use crate::data::schema::visitors::JsonSchemaVisitor;
use crate::data::schema::JsonSchema;
use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use serde::{Deserialize, Serialize};
use toy_pack::schema::{to_schema, Schema};

mod app;
mod layered;
mod plugin;
mod port_type;

pub use app::App;
pub use layered::Layered;
pub use plugin::Plugin;
pub use port_type::PortType;
use serde::de::DeserializeOwned;

/// Create layer.
/// Register a single service in a layered structure to compose a plugin.
pub fn layer<F>(layer: (&str, &str, F)) -> Layered<NoopEntry, F>
where
    F: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + Clone
        + 'static,
    F::Service: Send,
    F::Context: Send,
    F::Config: DeserializeOwned + Send,
{
    let (name_space, service_name, factory) = layer;
    Layered::<NoopEntry, F>::new(NoopEntry, name_space, service_name, factory)
}

/// Multiple layer structures are grouped together to form a plugin.
pub fn app<T>(registry: T) -> Plugin<NoopEntry, T>
where
    T: Registry,
{
    Plugin::new(NoopEntry, registry)
}

pub trait Registry: Clone + Send + Sync {
    fn service_types(&self) -> Vec<ServiceType>;

    fn schemas(&self) -> Vec<ServiceSchema>;

    fn delegate<T>(
        &self,
        tp: &ServiceType,
        uri: &Uri,
        executor: &mut T,
    ) -> Result<(), ServiceError>
    where
        T: ServiceExecutor<Request = Frame, Error = ServiceError, InitError = ServiceError>;
}

/// ServiceSchema (json schema format) for front-end api.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Registry for NoopEntry {
    fn service_types(&self) -> Vec<ServiceType> {
        Vec::new()
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        Vec::new()
    }

    fn delegate<T>(
        &self,
        tp: &ServiceType,
        _uri: &Uri,
        _executor: &mut T,
    ) -> Result<(), ServiceError>
    where
        T: ServiceExecutor<Request = Frame, Error = ServiceError, InitError = ServiceError>,
    {
        Err(ServiceError::service_not_found(tp))
    }
}
