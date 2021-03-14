use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use toy_api::graph::{self, GraphEntity, GraphsEntity};
use toy_api::supervisors::{self, Supervisor, Supervisors};
use toy_api::task::{self, PendingsEntity, TaskLogEntity, TasksEntity};

pub trait ApiClient {
    type Graph: GraphClient;
    type Task: TaskClient;
    type Supervisor: SupervisorClient;

    fn graph(&self) -> &Self::Graph;

    fn task(&self) -> &Self::Task;

    fn supervisor(&self) -> &Self::Supervisor;
}

#[async_trait]
pub trait GraphClient {
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
pub trait TaskClient {
    type WatchStream: Stream<Item = Result<PendingsEntity, ApiClientError>>;

    async fn watch(&self, opt: task::WatchOption) -> Result<Self::WatchStream, ApiClientError>;

    async fn post(&self, v: GraphEntity, opt: task::PostOption) -> Result<(), ApiClientError>;

    async fn list(&self, opt: task::ListOption) -> Result<TasksEntity, ApiClientError>;

    async fn log(&self, key: String, opt: task::LogOption)
        -> Result<TaskLogEntity, ApiClientError>;
}

#[async_trait]
pub trait SupervisorClient {
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
