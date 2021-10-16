use std::io::Read;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;

static CONFIG: &'static str = "./examples/tick.json";

fn main() {
    let _ = toy_tracing::console();

    let app = app(toy_plugin_commons::collect::all())
        .with(toy_plugin_commons::fanout::all())
        .with(toy_plugin_commons::stdio::all())
        .with(toy_plugin_commons::timer::all())
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

        let (sv, _, _) = toy::supervisor::local(ExecutorFactory, app);

        // supervisor start
        rt.block_on(async {
            let _ = sv.oneshot(g).await;
        });
    }
}
