use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct JoinHandle<T> {
    pub(crate) raw: tokio::task::JoinHandle<T>,
}

impl<T> JoinHandle<T> {
    pub fn abort(&self) {
        self.raw.abort()
    }

    pub fn is_finished(&self) -> bool {
        self.raw.is_finished()
    }
}

unsafe impl<T: Send> Send for JoinHandle<T> {}
unsafe impl<T: Send> Sync for JoinHandle<T> {}
impl<T> Unpin for JoinHandle<T> {}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.raw)
            .poll(cx)
            .map(|x| if x.is_ok() { Ok(x.unwrap()) } else { Err(()) })
    }
}
