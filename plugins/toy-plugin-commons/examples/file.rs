use std::io::Read;
use toy::core::prelude::*;
use toy::core::task::TaskId;
use toy::executor::ExecutorFactory;
use toy::supervisor::Request;
use tracing_subscriber::fmt::format::FmtSpan;

static CONFIG: &'static str = "./examples/file.yml";

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let app = app(toy_plugin_commons::load());

    let mut f = std::fs::File::open(CONFIG).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    if let Ok(config) = toy_pack_yaml::unpack::<Value>(s.as_str()) {
        let g = Graph::from(config).unwrap();
        // runtime for supervisor
        let mut rt = toy_rt::RuntimeBuilder::new()
            .worker_threads(4)
            .thread_name("toy-worker")
            .build()
            .unwrap();

        let (sv, mut tx, mut rx) = toy::supervisor::local(ExecutorFactory, app);

        // supervisor start
        rt.spawn(async {
            let _ = sv.run().await;
        });

        let _ = rt.block_on(async {
            let (tx2, rx2) = toy::core::oneshot::channel();
            let id = TaskId::new();
            let _ = tx.send_ok(Request::RunTask(id, g, tx2)).await;
            let uuid = rx2.recv().await;
            tracing::info!("task:{:?}", uuid);
        });

        tracing::info!("waiting shutdown reply from supervisor");
        let _ = rt.block_on(async {
            rx.next().await;
        });
    }
}
