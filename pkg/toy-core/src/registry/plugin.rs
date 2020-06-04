use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, NoopEntry, PluginRegistry, Registry, ServiceSchema};
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};
use toy_pack::deser::DeserializableOwned;
use toy_pack::schema::Schema;

#[derive(Clone)]
pub struct Plugin<S, F> {
    inner: Inner<S, F>,
    tps: Vec<ServiceSchema>,
}

#[derive(Clone)]
struct Inner<S, F> {
    tp: ServiceType,
    name_space: String,
    other: Option<S>,
    callback: F,
}

impl<S, F> Plugin<S, F> {
    pub fn new<R>(name_space: &str, service_name: &str, callback: F) -> Plugin<NoopEntry, F>
    where
        F: Fn() -> R + Clone,
        R: ServiceFactory,
        R::Config: Schema,
    {
        let s = ServiceSchema::new::<R::Config>(name_space, service_name);
        let tp = s.service_type.clone();
        let tps = vec![s];
        Plugin {
            inner: Inner {
                tp,
                name_space: name_space.to_string(),
                other: Option::<NoopEntry>::None,
                callback,
            },
            tps,
        }
    }

    pub fn service<F2, R>(self, service_name: &str, other: F2) -> Plugin<Self, F2>
    where
        Self: Sized,
        F2: Fn() -> R + Clone,
        R: ServiceFactory,
        R::Config: Schema,
    {
        let s = ServiceSchema::new::<R::Config>(&self.inner.name_space, service_name);
        let tp = s.service_type.clone();
        let mut tps = self.tps.clone();
        tps.push(s);
        Plugin {
            inner: Inner {
                tp,
                name_space: self.inner.name_space.to_string(),
                other: Some(self),
                callback: other,
            },
            tps,
        }
    }
}

impl<S, F> Registry for Plugin<S, F> {
    fn service_types(&self) -> Vec<ServiceType> {
        self.tps.iter().map(|x| x.service_type.clone()).collect()
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        self.tps.clone()
    }
}

impl<S, F> Debug for Plugin<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry {{ services:[{:?}] }}", self.service_types())
    }
}

impl<S, F, R> PluginRegistry for Plugin<S, F>
where
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
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

impl<S, F, R> Delegator for Plugin<S, F>
where
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
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
        match &self.inner.other {
            Some(other) => match other.delegate(tp, uri, executor) {
                Ok(_) => Ok(()),
                Err(_) => {
                    if self.inner.tp == *tp {
                        let f = (self.inner.callback)();
                        executor.spawn(tp, uri, f);
                        Ok(())
                    } else {
                        Err(ServiceError::service_not_found(tp))
                    }
                }
            },
            None => {
                if self.inner.tp == *tp {
                    let f = (self.inner.callback)();
                    executor.spawn(tp, uri, f);
                    Ok(())
                } else {
                    Err(ServiceError::service_not_found(tp))
                }
            }
        }
    }
}
