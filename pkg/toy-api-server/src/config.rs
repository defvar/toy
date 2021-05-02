//! Config for api server.

use crate::auth::Auth;
use crate::graph::store::GraphStore;
use crate::services::store::ServiceStore;
use crate::supervisors::store::SupervisorStore;
use crate::task::store::{TaskLogStore, TaskStore};

/// The traits that the config for api server implements.
pub trait ServerConfig<Http> {
    type Auth: Auth<Http> + 'static;
    type TaskLogStore: TaskLogStore<Http> + 'static;
    type TaskStore: TaskStore<Http> + 'static;
    type GraphStore: GraphStore<Http> + 'static;
    type SupervisorStore: SupervisorStore<Http> + 'static;
    type ServiceStore: ServiceStore<Http> + 'static;

    fn auth(&self) -> Self::Auth;

    fn task_store(&self) -> Self::TaskStore;

    fn task_log_store(&self) -> Self::TaskLogStore;

    fn graph_store(&self) -> Self::GraphStore;

    fn supervisor_store(&self) -> Self::SupervisorStore;

    fn service_store(&self) -> Self::ServiceStore;

    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;
}
