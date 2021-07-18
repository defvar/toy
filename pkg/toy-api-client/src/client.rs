//! # Traits for Toy Api Client

use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use toy_api::common;
use toy_api::graph::{Graph, GraphList};
use toy_api::role::{Role, RoleList};
use toy_api::role_binding::{RoleBinding, RoleBindingList};
use toy_api::services::{ServiceSpec, ServiceSpecList};
use toy_api::supervisors::{Supervisor, SupervisorList};
use toy_api::task::{
    self, AllocateRequest, AllocateResponse, PendingsEntity, TaskLogEntity, TasksEntity,
};

/// Composit All Api Client
pub trait ApiClient: Send + Sync {
    type Graph: GraphClient + 'static;
    type Task: TaskClient + 'static;
    type Supervisor: SupervisorClient + 'static;
    type Service: ServiceClient + 'static;
    type Rbac: Rbaclient + 'static;

    fn graph(&self) -> &Self::Graph;

    fn task(&self) -> &Self::Task;

    fn supervisor(&self) -> &Self::Supervisor;

    fn service(&self) -> &Self::Service;

    fn rbac(&self) -> &Self::Rbac;
}

/// Composit Rbac Api Client
pub trait Rbaclient: Send + Sync {
    type Role: RoleClient + 'static;
    type RoleBinding: RoleBindingClient + 'static;

    fn role(&self) -> &Self::Role;

    fn role_binding(&self) -> &Self::RoleBinding;
}

#[async_trait]
pub trait GraphClient: Send + Sync {
    async fn list(&self, opt: common::ListOption) -> Result<GraphList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: common::FindOption,
    ) -> Result<Option<Graph>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: Graph,
        opt: common::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: common::DeleteOption) -> Result<(), ApiClientError>;
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

    async fn post(&self, v: Graph, opt: task::PostOption) -> Result<(), ApiClientError>;

    async fn list(&self, opt: task::ListOption) -> Result<TasksEntity, ApiClientError>;

    async fn log(&self, key: String, opt: task::LogOption)
        -> Result<TaskLogEntity, ApiClientError>;
}

#[async_trait]
pub trait SupervisorClient: Send + Sync {
    async fn list(&self, opt: common::ListOption) -> Result<SupervisorList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: common::FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: Supervisor,
        opt: common::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: common::DeleteOption) -> Result<(), ApiClientError>;
}

#[async_trait]
pub trait ServiceClient: Send + Sync {
    async fn list(&self, opt: common::ListOption) -> Result<ServiceSpecList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: common::FindOption,
    ) -> Result<Option<ServiceSpec>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: ServiceSpec,
        opt: common::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: common::DeleteOption) -> Result<(), ApiClientError>;
}

#[async_trait]
pub trait RoleClient: Send + Sync {
    async fn list(&self, opt: common::ListOption) -> Result<RoleList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: common::FindOption,
    ) -> Result<Option<Role>, ApiClientError>;

    async fn put(&self, key: String, v: Role, opt: common::PutOption)
        -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: common::DeleteOption) -> Result<(), ApiClientError>;
}

#[async_trait]
pub trait RoleBindingClient: Send + Sync {
    async fn list(&self, opt: common::ListOption) -> Result<RoleBindingList, ApiClientError>;

    async fn find(
        &self,
        key: String,
        opt: common::FindOption,
    ) -> Result<Option<RoleBinding>, ApiClientError>;

    async fn put(
        &self,
        key: String,
        v: RoleBinding,
        opt: common::PutOption,
    ) -> Result<(), ApiClientError>;

    async fn delete(&self, key: String, opt: common::DeleteOption) -> Result<(), ApiClientError>;
}
