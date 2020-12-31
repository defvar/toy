use std::future::Future;

mod async_rt;

pub use async_rt::{Runtime, RuntimeBuilder, Spawner};

pub fn spawn<F>(future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let _ = tokio::spawn(future);
}

pub fn block_in_place<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    tokio::task::block_in_place(f)
}
