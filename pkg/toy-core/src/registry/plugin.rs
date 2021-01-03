use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, Layered, PluginRegistry, PortType, Registry, ServiceSchema};
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};
use toy_pack::deser::DeserializableOwned;
use toy_pack::schema::Schema;

/// One service as a plug-in.
#[derive(Clone)]
pub struct Plugin<F> {
    callback: F,
    schema: ServiceSchema,
}

impl<F> Plugin<F> {
    pub fn new<R>(
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
        let schema = ServiceSchema::new::<R::Config>(name_space, service_name, port_type);
        Plugin { callback, schema }
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

impl<F> Registry for Plugin<F> {
    fn service_types(&self) -> Vec<ServiceType> {
        vec![self.schema.service_type.clone()]
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        vec![self.schema.clone()]
    }
}

impl<F> Debug for Plugin<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plugin {{ services:[{:?}] }}", self.service_types())
    }
}

impl<F, R> PluginRegistry for Plugin<F>
where
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

impl<F, R> Delegator for Plugin<F>
where
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
        if self.schema.service_type == *tp {
            let f = (self.callback)();
            executor.spawn(tp, uri, f);
            Ok(())
        } else {
            Err(ServiceError::service_not_found(tp))
        }
    }
}
