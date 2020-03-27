use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::ServiceExecutor;
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

pub struct ServiceSet<S, F> {
    tp: ServiceType,
    other: S,
    callback: F,
}

pub trait Delegator {
    type Request;
    type Error: Error;
    type InitError: Error;

    fn delegate<T>(&self, tp: ServiceType, uri: Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >;
}

pub trait DelegatorExt: Delegator {
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

impl<T: Delegator> DelegatorExt for T {}

impl<F, R> Delegator for Registry<F>
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
    type InitError = ServiceError;

    fn delegate<T>(&self, tp: ServiceType, uri: Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >,
    {
        if self.tp == tp {
            let f = (self.callback)();
            executor.spawn(tp, uri, f);
            Ok(())
        } else {
            Err(ServiceError::error("don't know"))
        }
    }
}

impl<S, F, R> Delegator for ServiceSet<S, F>
where
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
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
    type InitError = ServiceError;

    fn delegate<T>(&self, tp: ServiceType, uri: Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >,
    {
        match self.other.delegate(tp.clone(), uri.clone(), executor) {
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
