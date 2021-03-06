use clap::Clap;
use toy::api_client::HttpApiClient;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
struct Opts {
    name: String,
    #[clap(short, long, default_value = "4")]
    worker: usize,
    #[clap(short, long, default_value = "supervisor")]
    thread_name_prefix: String,
    #[clap(short, long, default_value = "https://localhost:3030")]
    api_root: String,
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
        .thread_name(format!("{}-{}", opts.thread_name_prefix, opts.name))
        .worker_threads(opts.worker)
        .build()
        .unwrap();

    let app = app(toy_plugin_commons::load());

    let client = HttpApiClient::new(&opts.api_root).unwrap();
    let (sv, _tx, _rx) = toy::supervisor::spawn(opts.name, ExecutorFactory, app, client);

    rt.block_on(async {
        let _ = sv.run().await;
    });
}
