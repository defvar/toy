use futures::executor::{block_on, ThreadPool};
use futures::FutureExt;
use std::future::Future;
use std::io::Read;
use toy_core::prelude::*;
use toy_plugin_file::config::*;
use toy_plugin_file::service::*;
use toy_plugin_map::config::*;
use toy_plugin_map::service::*;

struct FutureRsRuntime {
    pool: ThreadPool,
}

impl AsyncRuntime for FutureRsRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.pool.spawn_ok(future.map(|_| ()));
    }
}

async fn go(graph: Graph) -> Result<(), ServiceError> {
    let c = plugin("write", factory!(write, FileWriteConfig, new_write_context))
        .service("read", factory!(read, FileReadConfig, new_read_context))
        .service(
            "mapping",
            factory!(mapping, MappingConfig, new_mapping_context),
        );

    let rt = FutureRsRuntime {
        pool: ThreadPool::new().unwrap(),
    };

    let e = Executor::new(&rt, graph);
    let _ = e.run(&c, Frame::default()).await;

    Ok(())
}

static CONFIG: &'static str = "./examples/file.yml";

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let mut f = std::fs::File::open(CONFIG).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    if let Ok(config) = toy_pack_yaml::unpack::<Value>(s.as_str()) {
        let g = Graph::from(config).unwrap();
        let _ = block_on(go(g));
    }
}
