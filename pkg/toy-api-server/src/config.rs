//! Config for api server.

use crate::auth::Auth;
use crate::graph::store::GraphStore;
use crate::task::store::{TaskLogStore, TaskStore};

/// The traits that the config for api server implements.
pub trait ServerConfig<Http> {
    type Auth: Auth<Http> + 'static;
    type TaskLogStore: TaskLogStore<Http> + 'static;
    type TaskStore: TaskStore<Http> + 'static;
    type GraphStore: GraphStore<Http> + 'static;

    fn auth(&self) -> Self::Auth;

    fn task_store(&self) -> Self::TaskStore;

    fn task_log_store(&self) -> Self::TaskLogStore;

    fn graph_store(&self) -> Self::GraphStore;
}
