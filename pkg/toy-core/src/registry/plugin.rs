use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::ServiceExecutor;
use crate::registry::{Delegator, NoopEntry, PluginRegistry, Registry};
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::fmt::{self, Debug};
use toy_pack::deser::DeserializableOwned;

#[derive(Clone)]
pub struct Plugin<S, F> {
    tp: ServiceType,
    other: Option<S>,
    callback: F,
    tps: Vec<ServiceType>,
}

impl<S, F> Plugin<S, F> {
    pub fn new<T>(tp: T, callback: F) -> Plugin<NoopEntry, F>
    where
        ServiceType: From<T>,
    {
        let tp: ServiceType = From::from(tp);
        let tps = vec![tp.clone()];
        Plugin {
            tp,
            other: Option::<NoopEntry>::None,
            callback,
            tps,
        }
    }

    pub fn service<F2, R, St>(self, tp: St, other: F2) -> Plugin<Self, F2>
    where
        Self: Sized,
        F2: Fn() -> R + Clone,
        ServiceType: From<St>,
    {
        let tp: ServiceType = From::from(tp);
        let mut tps = self.tps.clone();
        tps.push(tp.clone());
        Plugin {
            tp: tp.clone(),
            other: Some(self),
            callback: other,
            tps,
        }
    }
}

impl<S, F> Registry for Plugin<S, F> {
    fn service_types(&self) -> &Vec<ServiceType> {
        &self.tps
    }
}

impl<S, F> Debug for Plugin<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry {{ services:[{:?}] }}", self.tps)
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
        match &self.other {
            Some(other) => match other.delegate(tp, uri, executor) {
                Ok(_) => Ok(()),
                Err(_) => {
                    if self.tp == *tp {
                        let f = (self.callback)();
                        executor.spawn(tp, uri, f);
                        Ok(())
                    } else {
                        Err(ServiceError::service_not_found(tp))
                    }
                }
            },
            None => {
                if self.tp == *tp {
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
