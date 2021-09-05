use toy::api_server::authentication::CommonAuths;
use toy::api_server::task::btree_log_store::BTreeLogStore;
use toy::api_server::ServerConfig;
use toy_api_auth_firebase::FireAuth;
use toy_api_auth_jwt::ServiceAccountAuth;
use toy_api_store_etcd::EtcdStore;
use toy_h::impl_reqwest::ReqwestClient;

struct ToyConfig;

impl ServerConfig<ReqwestClient> for ToyConfig {
    type Auth = CommonAuths<FireAuth, ServiceAccountAuth>;
    type TaskLogStore = BTreeLogStore<ReqwestClient>;
    type TaskStore = EtcdStore<ReqwestClient>;
    type KvStore = EtcdStore<ReqwestClient>;

    fn auth(&self) -> Self::Auth {
        CommonAuths::new(FireAuth::new(), ServiceAccountAuth::new())
    }

    fn task_store(&self) -> Self::TaskStore {
        EtcdStore::new()
    }

    fn task_log_store(&self) -> Self::TaskLogStore {
        BTreeLogStore::new()
    }

    fn kv_store(&self) -> Self::KvStore {
        EtcdStore::new()
    }

    fn cert_path(&self) -> String {
        std::env::var("TOY_API_TLS_CERT_PATH").expect("config not found.")
    }

    fn key_path(&self) -> String {
        std::env::var("TOY_API_TLS_KEY_PATH").expect("config not found.")
    }

    fn pub_path(&self) -> String {
        std::env::var("TOY_API_TLS_PUB_PATH").expect("config not found.")
    }
}

fn main() {
    dotenv::dotenv().ok();
    let _ = toy_tracing::console();

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .worker_threads(2)
        .build()
        .unwrap();

    let client = ReqwestClient::new().unwrap();
    let server = toy::api_server::Server::new(ToyConfig).with_client(client);

    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030)).await;
    });
}
