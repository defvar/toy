use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use toy_api::graph::{self, GraphEntity, GraphsEntity};
use toy_api::services::{self, ServiceSpec, ServiceSpecList};
use toy_api::supervisors::{self, Supervisor, Supervisors};
use toy_api::task::{
    self, AllocateRequest, AllocateResponse, PendingsEntity, TaskLogEntity, TasksEntity,
};

pub trait ApiClient: Send + Sync {
    type Graph: GraphClient + 'static;
    type Task: TaskClient + 'static;
    type Supervisor: SupervisorClient + 'static;
    type Service: ServiceClient + 'static;

    fn graph(&self) -> &Self::Graph;

    fn task(&self) -> &Self::Task;

    fn supervisor(&self) -> &Self::Supervisor;

    fn service(&self) -> &Self::Service;
}

#[async_trait]
pub trait GraphClient: Send + Sync {
    async fn list(&self, opt: graph::ListOption) -> Result<GraphsEntity, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: graph::FindOption,
    ) -> Result<Option<GraphEntity>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: GraphEntity,
        opt: graph::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: graph::DeleteOption) -> Result<(), ApiClientError>;
}

#[async_trait]
pub trait TaskClient: Send + Sync {
    type WatchStream: Stream<Item = Result<PendingsEntity, ApiClientError>> + Send;

    async fn watch(&self, opt: task::WatchOption) -> Result<Self::WatchStream, ApiClientError>;

    async fn allocate(
        &self,
        key: String,
        req: AllocateRequest,
        opt: task::AllocateOption,
    ) -> Result<AllocateResponse, ApiClientError>;

    async fn post(&self, v: GraphEntity, opt: task::PostOption) -> Result<(), ApiClientError>;

    async fn list(&self, opt: task::ListOption) -> Result<TasksEntity, ApiClientError>;

    async fn log(&self, key: String, opt: task::LogOption)
        -> Result<TaskLogEntity, ApiClientError>;
}

#[async_trait]
pub trait SupervisorClient: Send + Sync {
    async fn list(&self, opt: supervisors::ListOption) -> Result<Supervisors, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: supervisors::FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: Supervisor,
        opt: supervisors::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(
        &self,
        key: String,
        opt: supervisors::DeleteOption,
    ) -> Result<(), ApiClientError>;
}

#[async_trait]
pub trait ServiceClient: Send + Sync {
    async fn list(&self, opt: services::ListOption) -> Result<ServiceSpecList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: services::FindOption,
    ) -> Result<Option<ServiceSpec>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: ServiceSpec,
        opt: services::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: services::DeleteOption) -> Result<(), ApiClientError>;
}
