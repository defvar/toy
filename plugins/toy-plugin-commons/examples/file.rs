use std::io::Read;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;

static CONFIG: &'static str = "./examples/file.yml";

fn main() {
    let _ = toy_tracing::console();
    let p = plugin(toy_plugin_commons::read())
        .layer(toy_plugin_commons::write())
        .layer(toy_plugin_commons::mapping())
        .layer(toy_plugin_commons::indexing())
        .layer(toy_plugin_commons::naming())
        .layer(toy_plugin_commons::put())
        .layer(toy_plugin_commons::reindexing())
        .layer(toy_plugin_commons::remove_by_index())
        .layer(toy_plugin_commons::remove_by_name())
        .layer(toy_plugin_commons::rename())
        .layer(toy_plugin_commons::single_value())
        .layer(toy_plugin_commons::to_map())
        .layer(toy_plugin_commons::to_seq())
        .layer(toy_plugin_commons::stdin())
        .layer(toy_plugin_commons::stdout())
        .layer(toy_plugin_commons::first())
        .layer(toy_plugin_commons::last())
        .layer(toy_plugin_commons::count())
        .layer(toy_plugin_commons::tick());

    let app = app(p);
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

        let (sv, _, _) = toy::supervisor::local(ExecutorFactory, app);

        rt.block_on(async {
            let _ = sv.oneshot(g).await;
        });
    }
}
