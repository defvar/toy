use crate::client::{
    ApiClient, GraphClient, Rbaclient, RoleBindingClient, RoleClient, ServiceClient,
    SupervisorClient, TaskClient,
};
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::common::{CommonPutResponse, DeleteOption, FindOption, ListOption, PutOption};
use toy_api::role::{Role, RoleList};
use toy_api::role_binding::{RoleBinding, RoleBindingList};
use toy_api::services::{ServiceSpec, ServiceSpecList, ServiceSpecListOption};
use toy_api::supervisors::{SupervisorBeatResponse, SupervisorListOption};
use toy_api::task::{PendingResult, TaskListOption};

#[derive(Clone)]
pub struct NoopApiClient;

#[derive(Clone)]
pub struct NoopRbacClient;

impl ApiClient for NoopApiClient {
    type Graph = NoopApiClient;
    type Task = NoopApiClient;
    type Supervisor = NoopApiClient;
    type Service = NoopApiClient;
    type Rbac = NoopRbacClient;

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

    fn rbac(&self) -> &Self::Rbac {
        unimplemented!()
    }
}

impl Rbaclient for NoopRbacClient {
    type Role = NoopApiClient;
    type RoleBinding = NoopApiClient;

    fn role(&self) -> &Self::Role {
        unimplemented!()
    }

    fn role_binding(&self) -> &Self::RoleBinding {
        unimplemented!()
    }
}

#[async_trait]
impl GraphClient for NoopApiClient {
    async fn list(&self, _opt: ListOption) -> Result<toy_api::graph::GraphList, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: FindOption,
    ) -> Result<Option<toy_api::graph::Graph>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: toy_api::graph::Graph,
        _opt: PutOption,
    ) -> Result<CommonPutResponse, ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl TaskClient for NoopApiClient {
    async fn post(
        &self,
        _v: toy_api::graph::Graph,
        _opt: toy_api::common::PostOption,
    ) -> Result<PendingResult, ApiClientError> {
        unimplemented!()
    }

    async fn list(&self, _opt: TaskListOption) -> Result<toy_api::task::Tasks, ApiClientError> {
        unimplemented!()
    }

    async fn log(
        &self,
        _key: String,
        _opt: toy_api::task::LogOption,
    ) -> Result<toy_api::task::TaskLog, ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl SupervisorClient for NoopApiClient {
    async fn list(
        &self,
        _opt: SupervisorListOption,
    ) -> Result<toy_api::supervisors::SupervisorList, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: FindOption,
    ) -> Result<Option<toy_api::supervisors::Supervisor>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: toy_api::supervisors::Supervisor,
        _opt: PutOption,
    ) -> Result<CommonPutResponse, ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }

    async fn beat(&self, _key: &str) -> Result<SupervisorBeatResponse, ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl ServiceClient for NoopApiClient {
    async fn list(&self, _opt: ServiceSpecListOption) -> Result<ServiceSpecList, ApiClientError> {
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
    ) -> Result<CommonPutResponse, ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl RoleClient for NoopApiClient {
    async fn list(&self, _opt: ListOption) -> Result<RoleList, ApiClientError> {
        unimplemented!()
    }

    async fn find(&self, _key: String, _opt: FindOption) -> Result<Option<Role>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: Role,
        _opt: PutOption,
    ) -> Result<CommonPutResponse, ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}

#[async_trait]
impl RoleBindingClient for NoopApiClient {
    async fn list(&self, _opt: ListOption) -> Result<RoleBindingList, ApiClientError> {
        unimplemented!()
    }

    async fn find(
        &self,
        _key: String,
        _opt: FindOption,
    ) -> Result<Option<RoleBinding>, ApiClientError> {
        unimplemented!()
    }

    async fn put(
        &self,
        _key: String,
        _v: RoleBinding,
        _opt: PutOption,
    ) -> Result<CommonPutResponse, ApiClientError> {
        unimplemented!()
    }

    async fn delete(&self, _key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        unimplemented!()
    }
}
