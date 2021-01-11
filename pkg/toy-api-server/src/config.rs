//! Config for api server.

use crate::auth::{Auth, NoAuth};
use crate::task::noop_store::NoopLogStore;
use crate::task::store::TaskLogStore;

/// The traits that the config for api server implements.
pub trait ServerConfig {
    type Auth: Auth + 'static;
    type TaskLogStore: TaskLogStore + 'static;

    fn auth(&self) -> Self::Auth;

    fn task_log_store(&self) -> Self::TaskLogStore;
}

/// Default config.
/// - No Auth.
/// - InMemory Config Store.
/// - Noop Log Store.
pub struct DefaultConfig;

impl ServerConfig for DefaultConfig {
    type Auth = NoAuth;
    type TaskLogStore = NoopLogStore;

    fn auth(&self) -> Self::Auth {
        NoAuth
    }

    fn task_log_store(&self) -> Self::TaskLogStore {
        NoopLogStore::new()
    }
}
