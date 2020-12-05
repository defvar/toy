use std::io::Read;
use std::time::Duration;
use toy_core::prelude::*;
use toy_core::supervisor::{Request, Supervisor};
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

        let (sv, mut tx, mut rx) = Supervisor::new(toy_rt::Spawner, app);

        // supervisor start
        rt.spawn(async {
            let _ = sv.run().await;
        });

        let _ = rt.block_on(async {
            let (tx2, rx2) = toy_core::oneshot::channel();
            let _ = tx.send_ok(Request::Task(g, tx2)).await;
            let uuid = rx2.recv().await;
            tracing::info!("task:{:?}", uuid);
        });

        std::thread::sleep(Duration::from_secs(5));

        tracing::info!("send shutdown request to supervisor");
        let _ = rt.block_on(async {
            let _ = tx.send_ok(Request::Shutdown).await;
        });

        tracing::info!("waiting shutdown reply from supervisor");
        let _ = rt.block_on(async {
            rx.next().await;
        });
    }
}
