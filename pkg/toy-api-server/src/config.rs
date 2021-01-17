//! Config for api server.

use crate::auth::{Auth, NoAuth};
use crate::graph::btree_store::BTreeStore;
use crate::graph::store::GraphStore;
use crate::task::noop_store::NoopLogStore;
use crate::task::store::TaskLogStore;
use toy_h::NoopHttpClient;

/// The traits that the config for api server implements.
pub trait ServerConfig<Http> {
    type Auth: Auth<Http> + 'static;
    type TaskLogStore: TaskLogStore<Http> + 'static;
    type GraphStore: GraphStore<Http> + 'static;

    fn auth(&self) -> Self::Auth;

    fn task_log_store(&self) -> Self::TaskLogStore;

    fn graph_store(&self) -> Self::GraphStore;
}

/// Default config.
/// - No Auth.
/// - InMemory Config Store.
/// - Noop Log Store.
pub struct DefaultConfig;

impl ServerConfig<NoopHttpClient> for DefaultConfig {
    type Auth = NoAuth<NoopHttpClient>;
    type TaskLogStore = NoopLogStore<NoopHttpClient>;
    type GraphStore = BTreeStore;

    fn auth(&self) -> Self::Auth {
        NoAuth::new()
    }

    fn task_log_store(&self) -> Self::TaskLogStore {
        NoopLogStore::new()
    }

    fn graph_store(&self) -> Self::GraphStore {
        BTreeStore::new()
    }
}
