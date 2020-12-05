use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, PluginRegistry, PortType, Registry, ServiceSchema};
use crate::service::ServiceFactory;
use crate::service_uri::Uri;
use crate::ServiceType;
use std::fmt::{self, Debug};
use toy_pack::deser::DeserializableOwned;
use toy_pack::schema::Schema;

#[derive(Clone)]
pub struct Layered<S, F> {
    callback: F,
    schema: ServiceSchema,
    other: S,
    schemas: Vec<ServiceSchema>,
}

impl<S, F> Layered<S, F> {
    pub fn new<R>(
        registry: S,
        name_space: &str,
        service_name: &str,
        port_type: PortType,
        other: F,
    ) -> Layered<S, F>
    where
        Self: Sized,
        S: Registry,
        F: Fn() -> R + Clone,
        R: ServiceFactory,
        R::Config: Schema,
    {
        let schema = ServiceSchema::new::<R::Config>(name_space, service_name, port_type);
        let mut schemas = registry.schemas().clone();
        schemas.push(schema.clone());
        Layered {
            callback: other,
            schema,
            other: registry,
            schemas,
        }
    }

    pub fn with<F2, R>(
        self,
        service_name: &str,
        port_type: PortType,
        other: F2,
    ) -> Layered<Self, F2>
    where
        Self: Sized,
        F2: Fn() -> R + Clone,
        R: ServiceFactory,
        R::Config: Schema,
    {
        let ns = self.schema.service_type.name_space().to_string();
        Layered::<Self, F2>::new(self, &ns, service_name, port_type, other)
    }
}

impl<S, F> Registry for Layered<S, F> {
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

impl<S, F> Debug for Layered<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry {{ services:[{:?}] }}", self.service_types())
    }
}

impl<S, F, R> PluginRegistry for Layered<S, F>
where
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError> + Clone,
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Service: Send,
    R::Context: Send,
    R::Config: DeserializableOwned + Send,
{
}

impl<S, F, R> Delegator for Layered<S, F>
where
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError> + Clone,
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Service: Send,
    R::Context: Send,
    R::Config: DeserializableOwned + Send,
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
        match self.other.delegate(tp, uri, executor) {
            Ok(_) => Ok(()),
            Err(_) => {
                if self.schema.service_type == *tp {
                    let f = (self.callback)();
                    executor.spawn(tp, uri, f);
                    Ok(())
                } else {
                    Err(ServiceError::service_not_found(tp))
                }
            }
        }
    }
}
