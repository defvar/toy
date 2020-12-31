use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy::supervisor::Supervisor;
use toy_api_auth_firebase::FireAuth;
use toy_api_store_etcd::EtcdStoreOpsFactory;
use tracing_subscriber::fmt::format::FmtSpan;

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

    // let mut api_rt = toy_rt::RuntimeBuilder::new()
    //     .thread_name("api-server")
    //     .worker_threads(2)
    //     .build()
    //     .unwrap();
    let mut api_rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .thread_name("api-server")
        .core_threads(2)
        .enable_all()
        .build()
        .unwrap();

    let app = app(toy_plugin_commons::load());

    let (sv, tx, rx) = Supervisor::new(ExecutorFactory, app);

    let server = toy_api_server::Server::new(EtcdStoreOpsFactory, FireAuth);

    sv_rt.spawn(async {
        let _ = sv.run().await;
    });
    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
    });
}
