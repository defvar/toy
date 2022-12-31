//! # toy-api-client Implementation for http

mod graph;
mod metrics;
mod role;
mod role_binding;
mod service;
mod supervisor;
mod task;

pub use graph::HttpGraphClient;
pub use service::HttpServiceClient;
pub use supervisor::HttpSupervisorClient;
pub use task::HttpTaskClient;

use crate::client::{ApiClient, Rbaclient};
use crate::error::ApiClientError;

use crate::http::metrics::HttpMetricsClient;
use crate::http::role::HttpRoleClient;
use crate::http::role_binding::HttpRoleBindingClient;
use std::sync::Arc;
use toy_api_http_common::auth::Auth;
use toy_h::impl_reqwest::ReqwestClient;

#[derive(Debug, Clone)]
pub struct HttpApiClient {
    graph: HttpGraphClient<ReqwestClient>,
    task: HttpTaskClient<ReqwestClient>,
    sv: HttpSupervisorClient<ReqwestClient>,
    service: HttpServiceClient<ReqwestClient>,
    rbac: HttpRbacClient,
    metrics: HttpMetricsClient<ReqwestClient>,
}

#[derive(Debug, Clone)]
pub struct HttpRbacClient {
    role: HttpRoleClient<ReqwestClient>,
    role_binding: HttpRoleBindingClient<ReqwestClient>,
}

impl HttpApiClient {
    pub fn new<P: AsRef<str>>(root: P, auth: Auth) -> Result<Self, ApiClientError> {
        let client = ReqwestClient::new()?;
        HttpApiClient::from(root, auth, &client)
    }

    pub fn from<P: AsRef<str>>(
        root: P,
        auth: Auth,
        inner: &ReqwestClient,
    ) -> Result<Self, ApiClientError> {
        let auth = Arc::new(auth);
        let rbac = HttpRbacClient::from(root.as_ref(), Arc::clone(&auth), inner.clone())?;
        Ok(Self {
            graph: HttpGraphClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
            task: HttpTaskClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
            sv: HttpSupervisorClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
            service: HttpServiceClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
            rbac,
            metrics: HttpMetricsClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
        })
    }
}

impl HttpRbacClient {
    pub fn from<P: AsRef<str>>(
        root: P,
        auth: Arc<Auth>,
        inner: ReqwestClient,
    ) -> Result<Self, ApiClientError> {
        Ok(Self {
            role: HttpRoleClient::new(root.as_ref(), Arc::clone(&auth), inner.clone()),
            role_binding: HttpRoleBindingClient::new(
                root.as_ref(),
                Arc::clone(&auth),
                inner.clone(),
            ),
        })
    }
}

impl ApiClient for HttpApiClient {
    type Graph = HttpGraphClient<ReqwestClient>;
    type Task = HttpTaskClient<ReqwestClient>;
    type Supervisor = HttpSupervisorClient<ReqwestClient>;
    type Service = HttpServiceClient<ReqwestClient>;
    type Rbac = HttpRbacClient;
    type Metrics = HttpMetricsClient<ReqwestClient>;

    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    fn task(&self) -> &Self::Task {
        &self.task
    }

    fn supervisor(&self) -> &Self::Supervisor {
        &self.sv
    }

    fn service(&self) -> &Self::Service {
        &self.service
    }

    fn rbac(&self) -> &Self::Rbac {
        &self.rbac
    }

    fn metrics(&self) -> &Self::Metrics {
        &self.metrics
    }
}

impl Rbaclient for HttpRbacClient {
    type Role = HttpRoleClient<ReqwestClient>;
    type RoleBinding = HttpRoleBindingClient<ReqwestClient>;

    fn role(&self) -> &Self::Role {
        &self.role
    }

    fn role_binding(&self) -> &Self::RoleBinding {
        &self.role_binding
    }
}
