use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::executor::{DefaultExecutor, ServiceExecutor};
use crate::service::{Service, ServiceFactory};
use crate::service_id::ServiceId;

#[derive(Clone)]
pub struct Registry<F> {
    id: ServiceId,
    callback: F,
}

impl<F> Registry<F> {
    pub fn new(id: ServiceId, callback: F) -> Registry<F> {
        Registry { id, callback }
    }
}

pub struct ServiceSet<T, F> {
    id: ServiceId,
    other: T,
    callback: F,
}

pub trait ServiceSpawner {
    type Request;
    type Error: Error;
    type InitError: Error;
    type ServiceExecutor: ServiceExecutor;

    fn spawn(&self, id: ServiceId, executor: &mut Self::ServiceExecutor)
        -> Result<(), Self::Error>;
}

pub trait ServiceSpawnerExt: ServiceSpawner {
    fn service<F, R>(self, id: ServiceId, other: F) -> ServiceSet<Self, F>
    where
        Self: Sized,
        F: Fn() -> R + Clone,
    {
        ServiceSet {
            id,
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
    <R as ServiceFactory>::Service: Send,
    <<R as ServiceFactory>::Service as Service>::Future: Send + 'static,
    R::Context: Send,
{
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;
    type ServiceExecutor = DefaultExecutor;

    fn spawn(
        &self,
        id: ServiceId,
        executor: &mut Self::ServiceExecutor,
    ) -> Result<(), Self::Error> {
        if self.id == id {
            let f = (self.callback)();
            executor.spawn(id, f);
            Ok(())
        } else {
            Err(ServiceError::error("don't know"))
        }
    }
}

impl<T, F, R> ServiceSpawner for ServiceSet<T, F>
where
    T: ServiceSpawner<
        Request = Frame,
        Error = ServiceError,
        InitError = ServiceError,
        ServiceExecutor = DefaultExecutor,
    >,
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Future: Send + 'static,
    <R as ServiceFactory>::Service: Send,
    <<R as ServiceFactory>::Service as Service>::Future: Send + 'static,
    R::Context: Send,
{
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;
    type ServiceExecutor = DefaultExecutor;

    fn spawn(
        &self,
        id: ServiceId,
        executor: &mut Self::ServiceExecutor,
    ) -> Result<(), Self::Error> {
        match self.other.spawn(id.clone(), executor) {
            Ok(_) => Ok(()),
            Err(_) => {
                if self.id == id {
                    let f = (self.callback)();
                    executor.spawn(id, f);
                    Ok(())
                } else {
                    Err(ServiceError::error("don't know"))
                }
            }
        }
    }
}
