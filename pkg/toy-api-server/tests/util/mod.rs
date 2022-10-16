use toy_api_server::authentication::NoAuth;
use toy_api_server::context::ServerState;
use toy_api_server::store::memory::MemoryStore;

#[derive(Clone)]
pub struct TestState {
    auth: NoAuth,
    task_log_store: MemoryStore,
    kv_store: MemoryStore,
}

impl ServerState for TestState {
    type Client = ();
    type Auth = NoAuth;
    type TaskLogStore = MemoryStore;
    type KvStore = MemoryStore;

    fn client(&self) -> &Self::Client {
        todo!()
    }

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }

    fn task_log_store(&self) -> &Self::TaskLogStore {
        &self.task_log_store
    }

    fn kv_store(&self) -> &Self::KvStore {
        &self.kv_store
    }
}

pub fn prepare() {
    std::env::set_var("TOY_AUTHORIZATION", "none");
}

pub fn test_state() -> impl ServerState {}
