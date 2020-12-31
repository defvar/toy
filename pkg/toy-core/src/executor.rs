use crate::data::Frame;
use crate::error::ServiceError;
use crate::node_channel::SignalOutgoings;
use crate::registry::Delegator;
use crate::service::ServiceFactory;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use crate::task::TaskContext;
use async_trait::async_trait;
use toy_pack::deser::DeserializableOwned;

pub trait ServiceExecutor {
    type Request;
    type Error;
    type InitError;

    fn spawn<F>(&mut self, service_type: &ServiceType, uri: &Uri, factory: F)
    where
        F: ServiceFactory<
            Request = Self::Request,
            Error = Self::Error,
            InitError = Self::InitError,
        > + Send
            + Sync
            + 'static,
        F::Service: Send,
        F::Context: Send,
        F::Config: DeserializableOwned + Send;
}

#[async_trait]
pub trait TaskExecutor {
    async fn run(
        self,
        delegator: impl Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>
            + Send
            + 'static,
        start_frame: Frame,
    ) -> Result<(), ServiceError>;
}

pub trait TaskExecutorFactory {
    type Executor: TaskExecutor + Send;
    fn new(ctx: TaskContext) -> (Self::Executor, SignalOutgoings);
}
