//! Traits for Task and Service Execution.
//!

use crate::data::Frame;
use crate::error::Error;
use crate::node_channel::SignalOutgoings;
use crate::registry::{App, Registry};
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use crate::task::TaskContext;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Trait for Service Executor.
///
/// Create and run a service using "factory".
pub trait ServiceExecutor {
    type Request;

    fn spawn<F>(&mut self, service_type: &ServiceType, uri: &Uri, factory: F)
    where
        F: ServiceFactory<Request = Self::Request> + Send + Sync + 'static,
        F::Service: Send,
        F::Context: Send,
        F::Config: DeserializeOwned + Send;
}

/// Trait for Task Executor.
///
/// Generate and execute tasks from Graph information.
/// This trait called from `Actor`.
#[async_trait]
pub trait TaskExecutor {
    type Error: Error + Send;

    async fn run<T>(self, app: &App<T>, start_frame: Frame) -> Result<(), Self::Error>
    where
        T: Registry;
}

/// Create a `TaskExecutor`.
pub trait TaskExecutorFactory {
    type Executor: TaskExecutor + Send;

    fn new(ctx: TaskContext) -> (Self::Executor, SignalOutgoings);
}
