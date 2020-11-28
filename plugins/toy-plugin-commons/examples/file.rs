use std::io::Read;
use std::time::Duration;
use toy_core::prelude::*;
use toy_core::registry::PortType;
use toy_core::supervisor::{Request, Supervisor};
use toy_plugin_file::config::*;
use toy_plugin_file::service::*;
use toy_plugin_map::config::*;
use toy_plugin_map::service::*;

static CONFIG: &'static str = "./examples/file.yml";

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let c = plugin(
        "example",
        "write",
        PortType::sink(),
        factory!(write, FileWriteConfig, new_write_context),
    )
    .with(
        "read",
        PortType::source(),
        factory!(read, FileReadConfig, new_read_context),
    )
    .with(
        "mapping",
        PortType::flow(),
        factory!(mapping, MappingConfig, new_mapping_context),
    );

    let app = app(c);

    let mut f = std::fs::File::open(CONFIG).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    if let Ok(config) = toy_pack_yaml::unpack::<Value>(s.as_str()) {
        let g = Graph::from(config).unwrap();
        // runtime for supervisor
        let mut rt = toy_rt::RuntimeBuilder::new()
            .threaded()
            .core_threads(4)
            .thread_name("toy-worker")
            .build()
            .unwrap();

        let (sv, mut tx, mut rx) = Supervisor::new(toy_rt::Spawner, app);

        // supervisor start
        rt.spawn(async {
            let _ = sv.run().await;
        });

        let _ = rt.block_on(async {
            let _ = tx.send_ok(Request::Task(g)).await;
        });

        std::thread::sleep(Duration::from_secs(3));

        log::info!("send shutdown request to supervisor");
        let _ = rt.block_on(async {
            let _ = tx.send_ok(Request::Shutdown).await;
        });

        log::info!("waiting shutdown reply from supervisor");
        let _ = rt.block_on(async {
            rx.next().await;
        });
    }
}
