use crate::data::schema::visitors::JsonSchemaVisitor;
use crate::data::schema::JsonSchema;
use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::ServiceExecutor;
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use toy_pack::schema::{to_schema, Schema};
use toy_pack::Pack;

mod app;
mod plugin;
pub use app::App;
pub use plugin::Plugin;

pub fn plugin<F, R>(name_space: &str, service_name: &str, callback: F) -> Plugin<NoopEntry, F>
where
    F: Fn() -> R + Clone,
    R: ServiceFactory,
    R::Config: Schema,
{
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

#[derive(Debug, Clone, Pack)]
pub struct ServiceSchema {
    tp: ServiceType,
    schema: Option<JsonSchema>,
}

impl ServiceSchema {
    pub fn new<T>(name_space: &str, service_name: &str) -> Self
    where
        T: Schema,
    {
        let tp: ServiceType = From::from(format!("{}.{}", name_space, service_name));
        let schema = to_schema::<T, JsonSchemaVisitor>(service_name, JsonSchemaVisitor)
            .map_err(|e| log::error!("an error occured; {:?}", e))
            .ok();
        Self { tp, schema }
    }
}

#[derive(Debug, Clone)]
pub struct NoopEntry;

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