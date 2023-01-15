extern crate core;

use std::future::Future;
use std::time::Duration;

mod async_rt;
mod join_handle;
mod metrics;

use crate::metrics::RuntimeMetrics;
pub use async_rt::{Runtime, RuntimeBuilder};
use join_handle::JoinHandle;

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let j = tokio::spawn(future);
    JoinHandle { raw: j }
}

pub fn spawn_named<F>(future: F, name: &str) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let j = tokio::task::Builder::new().name(name).spawn(future);
    JoinHandle { raw: j.unwrap() }
}

pub fn block_in_place<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    tokio::task::block_in_place(f)
}

pub fn sleep(millis: u64) -> impl Future<Output = ()> {
    tokio::time::sleep(Duration::from_millis(millis))
}

pub fn metrics() -> RuntimeMetrics {
    RuntimeMetrics::with(&tokio::runtime::Handle::current().metrics())
}
