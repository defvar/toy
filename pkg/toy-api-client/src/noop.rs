use crate::client::{ApiClient, GraphClient, ServiceClient, SupervisorClient, TaskClient};
use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use toy_api::services::{
    DeleteOption, FindOption, ListOption, PutOption, ServiceSpec, ServiceSpecList,
};
use toy_api::task::{AllocateOption, AllocateRequest, AllocateResponse};

#[derive(Clone)]
pub struct NoopApiClient;

impl ApiClient for NoopApiClient {
    type Graph = NoopApiClient;
    type Task = NoopApiClient;
    type Supervisor = NoopApiClient;
    type Service = NoopApiClient;

    fn graph(&self) -> &Self::Graph {
        unimplemented!()
    }

    fn task(&self) -> &Self::Task {
        unimplemented!()
    }

    fn supervisor(&self) -> &Self::Supervisor {
        unimplemented!()
    }

    fn service(&self) -> &Self::Service {
        unimplemented!()
    }
}

#[async_trait]
impl GraphClient for NoopApiClient {
    async fn list(
        &self,
        _opt: toy_api::graph::ListOption,
    ) -> Result<toy_api::graph::GraphsEntity, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: toy_api::graph::FindOption,
    ) -> Result<Option<toy_api::graph::GraphEntity>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: toy_api::graph::GraphEntity,
        _opt: toy_api::graph::PutOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }

    async fn delete(
        &self,
        _key: String,
        _opt: toy_api::graph::DeleteOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl TaskClient for NoopApiClient {
    type WatchStream = impl Stream<Item = Result<toy_api::task::PendingsEntity, ApiClientError>>;

    async fn watch(
        &self,
        _opt: toy_api::task::WatchOption,
    ) -> Result<Self::WatchStream, ApiClientError> {
        Ok(futures_util::stream::empty())
    }

    async fn allocate(
        &self,
        _key: String,
        _req: AllocateRequest,
        _opt: AllocateOption,
    ) -> Result<AllocateResponse, ApiClientError> {
        unimplemented!()
    }

    async fn post(
        &self,
        _v: toy_api::graph::GraphEntity,
        _opt: toy_api::task::PostOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }

    async fn list(
        &self,
        _opt: toy_api::task::ListOption,
    ) -> Result<toy_api::task::TasksEntity, ApiClientError> {
        unimplemented!()
    }

    async fn log(
        &self,
        _key: String,
        _opt: toy_api::task::LogOption,
    ) -> Result<toy_api::task::TaskLogEntity, ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl SupervisorClient for NoopApiClient {
    async fn list(
        &self,
        _opt: toy_api::supervisors::ListOption,
    ) -> Result<toy_api::supervisors::Supervisors, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: toy_api::supervisors::FindOption,
    ) -> Result<Option<toy_api::supervisors::Supervisor>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: toy_api::supervisors::Supervisor,
        _opt: toy_api::supervisors::PutOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }

    async fn delete(
        &self,
        _key: String,
        _opt: toy_api::supervisors::DeleteOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl ServiceClient for NoopApiClient {
    async fn list(&self, _opt: ListOption) -> Result<ServiceSpecList, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: FindOption,
    ) -> Result<Option<ServiceSpec>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: ServiceSpec,
        _opt: PutOption,
    ) -> Result<(), ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}
