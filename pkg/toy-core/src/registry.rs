use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::{DefaultExecutor, ServiceExecutor};
use crate::service::{Service, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use toy_pack::deser::DeserializableOwned;

#[derive(Clone)]
pub struct Registry<F> {
    tp: ServiceType,
    callback: F,
}

impl<F> Registry<F> {
    pub fn new<T>(tp: T, callback: F) -> Registry<F>
    where
        ServiceType: From<T>,
    {
        Registry {
            tp: From::from(tp),
            callback,
        }
    }
}

pub struct ServiceSet<T, F> {
    tp: ServiceType,
    other: T,
    callback: F,
}

pub trait ServiceSpawner {
    type Request;
    type Error: Error;
    type ServiceExecutor: ServiceExecutor;

    fn spawn(
        &self,
        tp: ServiceType,
        uri: Uri,
        executor: &mut Self::ServiceExecutor,
    ) -> Result<(), Self::Error>;
}

pub trait ServiceSpawnerExt: ServiceSpawner {
    fn service<F, R, St>(self, tp: St, other: F) -> ServiceSet<Self, F>
    where
        Self: Sized,
        F: Fn() -> R + Clone,
        ServiceType: From<St>,
    {
        ServiceSet {
            tp: From::from(tp),
            other: self,
            callback: other,
        }
    }
}

impl<T: ServiceSpawner> ServiceSpawnerExt for T {}

impl<F, R> ServiceSpawner for Registry<F>
where
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Future: Send + 'static,
    R::Service: Send,
    <<R as ServiceFactory>::Service as Service>::Future: Send + 'static,
    R::Context: Send,
    R::Config: DeserializableOwned<Value = R::Config> + Send,
{
    type Request = Frame;
    type Error = ServiceError;
    type ServiceExecutor = DefaultExecutor;

    fn spawn(
        &self,
        tp: ServiceType,
        uri: Uri,
        executor: &mut Self::ServiceExecutor,
    ) -> Result<(), Self::Error> {
        if self.tp == tp {
            let f = (self.callback)();
            executor.spawn(tp, uri, f);
            Ok(())
        } else {
            Err(ServiceError::error("don't know"))
        }
    }
}

impl<T, F, R> ServiceSpawner for ServiceSet<T, F>
where
    T: ServiceSpawner<Request = Frame, Error = ServiceError, ServiceExecutor = DefaultExecutor>,
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Future: Send + 'static,
    R::Service: Send,
    <<R as ServiceFactory>::Service as Service>::Future: Send + 'static,
    R::Context: Send,
    R::Config: DeserializableOwned<Value = R::Config> + Send,
{
    type Request = Frame;
    type Error = ServiceError;
    type ServiceExecutor = DefaultExecutor;

    fn spawn(
        &self,
        tp: ServiceType,
        uri: Uri,
        executor: &mut Self::ServiceExecutor,
    ) -> Result<(), Self::Error> {
        match self.other.spawn(tp.clone(), uri.clone(), executor) {
            Ok(_) => Ok(()),
            Err(_) => {
                if self.tp == tp {
                    let f = (self.callback)();
                    executor.spawn(tp, uri, f);
                    Ok(())
                } else {
                    Err(ServiceError::error("don't know"))
                }
            }
        }
    }
}
