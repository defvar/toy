use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::ServiceExecutor;
use crate::service::{Service, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};
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

    pub fn service<F2, R, St>(self, tp: St, other: F2) -> ServiceSet<Self, F2>
    where
        Self: Sized,
        F2: Fn() -> R + Clone,
        ServiceType: From<St>,
    {
        let tp: ServiceType = From::from(tp);
        let self_tp = self.tp.clone();
        ServiceSet {
            tp: tp.clone(),
            other: self,
            callback: other,
            tps: vec![self_tp, tp],
        }
    }
}

pub struct ServiceSet<S, F> {
    tp: ServiceType,
    other: S,
    callback: F,
    // debug use
    tps: Vec<ServiceType>,
}

impl<S, F> ServiceSet<S, F> {
    pub fn service<F2, R, St>(self, tp: St, other: F2) -> ServiceSet<Self, F2>
    where
        Self: Sized,
        F2: Fn() -> R + Clone,
        ServiceType: From<St>,
    {
        let tp: ServiceType = From::from(tp);
        let mut tps = self.tps.clone();
        tps.push(tp.clone());
        ServiceSet {
            tp: tp.clone(),
            other: self,
            callback: other,
            tps,
        }
    }
}

impl<F> Debug for Registry<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry {{ services:[{:?}] }}", self.tp)
    }
}

impl<S, F> Debug for ServiceSet<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServiceSet {{ services:{:?} }}", self.tps)
    }
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

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> Result<(), Self::Error>
    where
        T: ServiceExecutor<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        >,
    {
        if self.tp == *tp {
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
                if self.tp == *tp {
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
