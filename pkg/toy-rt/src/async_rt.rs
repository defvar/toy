use std::future::Future;
use std::io;
use tokio::runtime::{Builder, Runtime as TokioRuntime};

pub struct Runtime {
    rt: TokioRuntime,
}

pub struct RuntimeBuilder {
    builder: Builder,
}

impl Runtime {
    pub fn block_on<F>(&mut self, future: F) -> F::Output
    where
        F: Future,
    {
        self.rt.block_on(future)
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let _ = self.rt.spawn(future);
    }

    pub fn spawn_named<F>(&self, future: F, name: &str)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let _ = tokio::task::Builder::new()
            .name(name)
            .spawn_on(future, self.rt.handle());
    }
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            builder: Builder::new_multi_thread(),
        }
    }

    pub fn worker_threads(&mut self, v: usize) -> &mut RuntimeBuilder {
        self.builder.worker_threads(v);
        self
    }

    pub fn thread_name(&mut self, v: impl Into<String>) -> &mut RuntimeBuilder {
        self.builder.thread_name(v);
        self
    }

    pub fn build(&mut self) -> Result<Runtime, io::Error> {
        let rt = self.builder.enable_all().build()?;
        Ok(Runtime { rt })
    }
}
