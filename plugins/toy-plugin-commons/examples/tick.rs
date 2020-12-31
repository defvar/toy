use std::io::Read;
use toy::core::prelude::*;
use toy::supervisor::{Request, Supervisor};
use tracing_subscriber::fmt::format::FmtSpan;

static CONFIG: &'static str = "./examples/tick.json";

fn main() {
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(time)
        .init();

    let app = app(toy_plugin_commons::load());

    let mut f = std::fs::File::open(CONFIG).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    if let Ok(config) = toy_pack_json::unpack::<Value>(s.as_bytes()) {
        let g = Graph::from(config).unwrap();
        tracing::info!("g:{:?}", g);
        // runtime for supervisor
        let mut rt = toy_rt::RuntimeBuilder::new()
            .worker_threads(4)
            .thread_name("toy-worker")
            .build()
            .unwrap();

        let (sv, mut tx, mut rx) = Supervisor::new(app);

        // supervisor start
        rt.spawn(async {
            let _ = sv.run().await;
        });

        let _ = rt.block_on(async {
            let (tx2, rx2) = toy::core::oneshot::channel();
            let _ = tx.send_ok(Request::RunTask(g, tx2)).await;
            let uuid = rx2.recv().await;
            tracing::info!("task:{:?}", uuid);
        });

        // tracing::info!("send shutdown request to supervisor");
        // let _ = rt.block_on(async {
        //     let _ = tx.send_ok(Request::Shutdown).await;
        // });

        tracing::info!("waiting shutdown reply from supervisor");
        let _ = rt.block_on(async {
            rx.next().await;
        });
    }
}
