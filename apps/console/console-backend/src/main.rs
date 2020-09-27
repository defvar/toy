use toy_api_store_etcd::EtcdStoreOpsFactory;
use toy_core::prelude::*;
use toy_core::supervisor::Supervisor;

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

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

    let regi = app(toy_plugin_file::load()).plugin(toy_plugin_map::load());

    let service_rt = toy_rt::RuntimeBuilder::new()
        .thread_name("service-worker")
        .threaded()
        .build()
        .unwrap();

    let (sv, tx, rx) = Supervisor::new(service_rt, regi);

    let server = toy_api_server::Server::new(EtcdStoreOpsFactory);

    sv_rt.spawn(async {
        let _ = sv.run().await;
    });
    api_rt.block_on(async move {
        let _ = server.run(([127, 0, 0, 1], 3030), tx, rx).await;
    });
}
