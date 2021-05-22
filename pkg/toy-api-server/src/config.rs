//! Config for api server.

use crate::authentication::Auth;
use crate::store::kv::KvStore;
use crate::task::store::{TaskLogStore, TaskStore};

/// The traits that the config for api server implements.
pub trait ServerConfig<Http> {
    type Auth: Auth<Http> + 'static;
    type TaskLogStore: TaskLogStore<Http> + 'static;
    type TaskStore: TaskStore<Http> + 'static;
    type KvStore: KvStore<Http> + 'static;

    fn auth(&self) -> Self::Auth;

    fn task_store(&self) -> Self::TaskStore;

    fn task_log_store(&self) -> Self::TaskLogStore;

    fn kv_store(&self) -> Self::KvStore;

    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;
}
