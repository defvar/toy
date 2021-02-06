#![feature(type_alias_impl_trait)]

use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy::supervisor::Supervisor;
use toy_api_server::auth::NoAuth;
use toy_api_server::task::btree_log_store::BTreeLogStore;
use toy_api_server::ServerConfig;
use toy_api_store_etcd::EtcdStore;
use toy_h::impl_reqwest::ReqwestClient;
use tracing_subscriber::fmt::format::FmtSpan;

struct ToyConfig;

impl ServerConfig<ReqwestClient> for ToyConfig {
    type Auth = NoAuth<ReqwestClient>;
    type TaskLogStore = BTreeLogStore<ReqwestClient>;
    type TaskStore = EtcdStore<ReqwestClient>;
    type GraphStore = EtcdStore<ReqwestClient>;
    type SupervisorStore = EtcdStore<ReqwestClient>;

    fn auth(&self) -> Self::Auth {
        NoAuth::new()
    }

    fn task_store(&self) -> Self::TaskStore {
        EtcdStore::new()
    }

    fn task_log_store(&self) -> Self::TaskLogStore {
        BTreeLogStore::new()
    }

    fn graph_store(&self) -> Self::GraphStore {
        EtcdStore::new()
    }

    fn supervisor_store(&self) -> Self::SupervisorStore {
        EtcdStore::new()
    }

    fn cert_path(&self) -> String {
        std::env::var("TOY_API_CERT_PATH").expect("config not found.")
    }

    fn key_path(&self) -> String {
        std::env::var("TOY_API_KEY_PATH").expect("config not found.")
    }
}

fn main() {
    dotenv::dotenv().ok();
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(time)
        .init();

    let sv_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("supervisor")
        .worker_threads(4)
        .build()
        .unwrap();

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .worker_threads(2)
        .build()
        .unwrap();

    let app = app(toy_plugin_commons::load());

    let (sv, tx, rx) = Supervisor::new(ExecutorFactory, app);

    let client = ReqwestClient::new().unwrap();
    let server = toy_api_server::Server::new(ToyConfig).with_client(client);

    sv_rt.spawn(async {
        let _ = sv.run().await;
    });
    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
    });
}
