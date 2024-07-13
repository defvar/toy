use std::io::Read;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy::supervisor::exporters::NoopExporter;
use toy::supervisor::SupervisorConfig;

static CONFIG: &'static str = "./examples/stat.json";

#[derive(Clone)]
struct SVConfig;

impl SupervisorConfig for SVConfig {
    type EventExporter = NoopExporter;
    type MetricsExporter = NoopExporter;

    fn heart_beat_interval_mills(&self) -> u64 {
        todo!()
    }

    fn event_export_interval_mills(&self) -> u64 {
        todo!()
    }

    fn cert_path(&self) -> String {
        todo!()
    }

    fn key_path(&self) -> String {
        todo!()
    }

    fn pub_path(&self) -> String {
        todo!()
    }

    fn metrics_exporter(&self) -> Self::MetricsExporter {
        todo!()
    }

    fn event_exporter(&self) -> Self::EventExporter {
        todo!()
    }
}

fn main() {
    let _ = toy_tracing::console();

    let app = app(toy_plugin_commons::stat::all())
        .with(toy_plugin_commons::fanout::all())
        .with(toy_plugin_commons::stdio::all())
        .with(toy_plugin_commons::timer::all())
        .with(toy_plugin_commons::file::all())
        .build();

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

        let (sv, _, _) = toy::supervisor::local(ExecutorFactory, app, SVConfig);

        // supervisor start
        rt.block_on(async {
            let _ = sv.oneshot(g).await;
        });
    }
}
