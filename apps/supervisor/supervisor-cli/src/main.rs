use clap::Clap;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy_supervisor::Supervisor;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
struct Opts {
    #[clap(short, long, default_value = "4")]
    worker: usize,
    #[clap(default_value = "supervisor")]
    thread_name: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    dotenv::dotenv().ok();

    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(time)
        .init();

    tracing::info!("start cli for config:{:?}", opts);

    let mut rt = toy_rt::RuntimeBuilder::new()
        .thread_name(opts.thread_name)
        .worker_threads(opts.worker)
        .build()
        .unwrap();

    let app = app(toy_plugin_commons::load());

    let (sv, mut tx, mut rx) = Supervisor::new(ExecutorFactory, app);

    rt.spawn(async {
        let _ = sv.run().await;
    });

    tracing::info!("waiting shutdown reply from supervisor");
    let _ = rt.block_on(async {
        rx.next().await;
    });
}
