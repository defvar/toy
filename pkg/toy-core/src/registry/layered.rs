use crate::data::Frame;
use crate::executor::ServiceExecutor;
use crate::registry::{ExecuteResult, Registry, ServiceSchema};
use crate::service::{Service, ServiceFactory};
use crate::service_uri::Uri;
use crate::ServiceType;
use serde::de::DeserializeOwned;
use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct Layered<L, R> {
    other: L,
    factory: R,
    schema: ServiceSchema,
    schemas: Vec<ServiceSchema>,
}

impl<L, R> Layered<L, R>
where
    Self: Sized,
    L: Registry,
    R: ServiceFactory<Request = Frame> + Send + Sync + Clone + 'static,
    R::Service: Send,
    R::Context: Send,
    R::Config: DeserializeOwned + Send,
{
    pub fn new(init: L, name_space: &str, service_name: &str, factory: R) -> Layered<L, R> {
        let schema =
            ServiceSchema::new::<R::Config>(name_space, service_name, R::Service::port_type());
        let mut schemas = init.schemas().clone();
        schemas.push(schema.clone());
        Layered {
            factory,
            schema,
            other: init,
            schemas,
        }
    }

    pub fn layer<F>(self, layer: (&str, &str, F)) -> Layered<Self, F>
    where
        F: ServiceFactory<Request = Frame> + Send + Sync + Clone + 'static,
        F::Service: Send,
        F::Context: Send,
        F::Config: DeserializeOwned + Send,
    {
        let (name_space, service_name, factory) = layer;
        Layered::<Self, F>::new(self, name_space, service_name, factory)
    }
}

impl<S, F> Registry for Layered<S, F>
where
    S: Registry,
    F: ServiceFactory<Request = Frame> + Send + Sync + Clone + 'static,
    F::Service: Send,
    F::Context: Send,
    F::Config: DeserializeOwned + Send,
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

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> ExecuteResult
    where
        T: ServiceExecutor<Request = Frame>,
    {
        match self.other.delegate(tp, uri, executor) {
            ExecuteResult::Done => ExecuteResult::Done,
            ExecuteResult::NotFound => {
                if self.schema.service_type == *tp {
                    let f = self.factory.clone();
                    executor.spawn(tp, uri, f);
                    ExecuteResult::Done
                } else {
                    ExecuteResult::NotFound
                }
            }
        }
    }
}

impl<S, F> Debug for Layered<S, F>
where
    S: Registry,
    F: ServiceFactory<Request = Frame> + Send + Sync + Clone + 'static,
    F::Service: Send,
    F::Context: Send,
    F::Config: DeserializeOwned + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry {{ services:[{:?}] }}", self.service_types())
    }
}
