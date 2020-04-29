use std::future::Future;
use std::io;
use tokio::runtime::{Builder, Runtime as TokioRuntime};
use toy_core::executor::AsyncRuntime;

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

    // pub fn spawn<F>(&self, future: F) -> impl Future<Output = Result<F::Output, io::Error>>
    // where
    //     F: Future + Send + 'static,
    //     F::Output: Send + 'static,
    // {
    //     self.rt.spawn(future)
    // }
}

impl AsyncRuntime for Runtime {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let _ = self.rt.spawn(future);
    }
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            builder: Builder::new(),
        }
    }

    pub fn basic(&mut self) -> &mut RuntimeBuilder {
        self.builder.basic_scheduler().enable_all();
        self
    }

    pub fn threaded(&mut self) -> &mut RuntimeBuilder {
        self.builder.threaded_scheduler().enable_all();
        self
    }

    pub fn core_threads(&mut self, v: usize) -> &mut RuntimeBuilder {
        self.builder.core_threads(v);
        self
    }

    pub fn max_threads(&mut self, v: usize) -> &mut RuntimeBuilder {
        self.builder.max_threads(v);
        self
    }

    pub fn thread_name(&mut self, v: impl Into<String>) -> &mut RuntimeBuilder {
        self.builder.thread_name(v);
        self
    }

    pub fn build(&mut self) -> Result<Runtime, io::Error> {
        let rt = self.builder.build()?;
        Ok(Runtime { rt })
    }
}
