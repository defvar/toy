use futures::executor::{block_on, ThreadPool};
use futures::FutureExt;
use std::future::Future;
use toy_core::prelude::*;
use toy_plugin_file::config::*;
use toy_plugin_file::service::*;

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

fn graph() -> Graph {
    let mut r = Map::new();
    let mut kind = Map::new();
    kind.insert("File".to_string(), Value::None);
    r.insert("type".to_string(), Value::from("read".to_string()));
    r.insert("uri".to_string(), Value::from("reader".to_string()));
    r.insert("kind".to_string(), Value::from(kind.clone()));
    r.insert("path".to_string(), Value::from(IN.to_string()));
    r.insert("wires".to_string(), Value::from("writer".to_string()));
    let r = Value::from(r);

    let mut w = Map::new();
    w.insert("type".to_string(), Value::from("write".to_string()));
    w.insert("uri".to_string(), Value::from("writer".to_string()));
    w.insert("kind".to_string(), Value::from(kind.clone()));
    w.insert("path".to_string(), Value::from(OUT.to_string()));
    w.insert("wires".to_string(), Value::None);
    let w = Value::from(w);

    let seq = Value::Seq(vec![r, w]);

    let mut services = Map::new();
    services.insert("services".to_string(), seq);

    Graph::from(Value::Map(services)).unwrap()
}

async fn go() -> Result<(), ServiceError> {
    let c = Registry::new("write", factory!(write, FileWriteConfig, new_write_context))
        .service("read", factory!(read, FileReadConfig, new_read_context));

    let rt = FutureRsRuntime {
        pool: ThreadPool::new().unwrap(),
    };

    let g = graph();
    let e = Executor::new(rt, g);
    let _ = e.run(c, Frame::default()).await;

    Ok(())
}

static IN: &'static str = "./examples/file.csv";
static OUT: &'static str = "./examples/file.out.rs.bk";

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let _ = block_on(go());
}
