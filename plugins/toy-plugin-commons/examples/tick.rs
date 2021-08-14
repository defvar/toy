use std::io::Read;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use tracing_subscriber::fmt::format::FmtSpan;

static CONFIG: &'static str = "./examples/tick.json";

fn main() {
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
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

        let (sv, _, _) = toy::supervisor::local(ExecutorFactory, app);

        // supervisor start
        rt.block_on(async {
            let _ = sv.oneshot(g).await;
        });
    }
}
