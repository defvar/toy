use std::net::SocketAddr;
use toy::api_server::authentication::CommonAuths;
use toy::api_server::task::btree_log_store::BTreeLogStore;
use toy::api_server::ServerConfig;
use toy_api_auth_jwt::JWTAuth;
use toy_api_store_etcd::EtcdStore;
use toy_h::impl_reqwest::ReqwestClient;
use toy_tracing::{LogRotation, CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT};

#[derive(Clone)]
struct ToyConfig;

impl ServerConfig<ReqwestClient> for ToyConfig {
    type Auth = CommonAuths<JWTAuth, JWTAuth>;
    type TaskLogStore = BTreeLogStore<ReqwestClient>;
    type TaskStore = EtcdStore<ReqwestClient>;
    type KvStore = EtcdStore<ReqwestClient>;

    fn auth(&self) -> Self::Auth {
        CommonAuths::new(JWTAuth::new(), JWTAuth::new())
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

    let tokio_console_host = std::env::var("TOY_API_TOKIO_CONSOLE_HOST");
    let tokio_console_port = std::env::var("TOY_API_TOKIO_CONSOLE_PORT");

    let addr = match (tokio_console_host, tokio_console_port) {
        (Ok(host), Ok(port)) => format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        (Ok(host), Err(_)) => format!("{}:{}", host, CONSOLE_DEFAULT_PORT)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        (Err(_), Ok(port)) => format!("{}:{}", CONSOLE_DEFAULT_IP, port)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        _ => SocketAddr::new(CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT),
    };

    let _ = toy_tracing::console_with_addr(addr);

    let host = std::env::var("TOY_API_HOST").expect("env not found. TOY_API_HOST");
    let port = std::env::var("TOY_API_PORT").expect("env not found. TOY_API_PORT");

    let host = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("invalid IP Address.");

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .worker_threads(2)
        .build()
        .unwrap();

    let client = ReqwestClient::new().unwrap();
    let server = toy::api_server::Server::new(ToyConfig).with_client(client);

    api_rt.block_on(async move {
        tracing::info!("start api-server :{}", host);
        tracing::info!("tokio tracing console :{}", addr);
        let _ = server.run(host).await;
    });
}
