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
    let r = map_value! {
      "type" => "read",
      "uri" => "reader",
      "kind" => "File",
      "path" => IN.to_string(),
      "wires" => "writer"
    };

    let w = map_value! {
      "type" => "write",
      "uri" => "writer",
      "kind" => "File",
      "path" => OUT.to_string(),
      "wires" => Value::None
    };

    let seq = seq_value![r, w];

    let services = map_value! {
      "services" => seq
    };

    Graph::from(services).unwrap()
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
