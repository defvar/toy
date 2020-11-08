use toy_api_auth_firebase::FireAuth;
use toy_api_store_etcd::EtcdStoreOpsFactory;
use toy_core::prelude::*;
use toy_core::supervisor::Supervisor;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let sv_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("supervisor")
        .threaded()
        .core_threads(1)
        .build()
        .unwrap();

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("api-server")
        .threaded()
        .core_threads(1)
        .build()
        .unwrap();

    let regi = app(toy_plugin_file::load())
        .plugin(toy_plugin_map::load())
        .plugin(toy_plugin_fanout::load());

    let service_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("service-worker")
        .threaded()
        .build()
        .unwrap();

    let (sv, tx, rx) = Supervisor::new(service_rt, regi);

    let server = toy_api_server::Server::new(EtcdStoreOpsFactory, FireAuth);

    sv_rt.spawn(async {
        let _ = sv.run().await;
    });
    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
    });
}
