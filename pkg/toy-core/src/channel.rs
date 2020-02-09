use crate::error::Error;
use futures::channel::mpsc::{self, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::{Future, FutureExt};

pub fn stream<T, Err>(buffer: usize) -> (Outgoing<T, Err>, Incoming<T, Err>) {
    let (tx, rx) = mpsc::channel(buffer);
    (Outgoing::new(tx), Incoming::new(rx))
}

pub struct Incoming<T, Err> {
    inner: Receiver<Result<T, Err>>,
}

impl<T, Err> Incoming<T, Err> {
    pub fn new(rx: Receiver<Result<T, Err>>) -> Incoming<T, Err> {
        Incoming { inner: rx }
    }

    pub async fn next(&mut self) -> Option<Result<T, Err>> {
        StreamExt::next(&mut self.inner).await
    }

    pub async fn for_each<Fut, F>(self, f: F) -> ()
    where
        F: FnMut(Result<T, Err>) -> Fut,
        Fut: Future<Output = ()>,
        Self: Sized,
    {
        StreamExt::for_each(self.inner, f).await
    }
}

pub struct Outgoing<T, Err> {
    inner: Sender<Result<T, Err>>,
}

impl<T, Err> Outgoing<T, Err> {
    pub fn new(tx: Sender<Result<T, Err>>) -> Outgoing<T, Err> {
        Outgoing { inner: tx }
    }
}

impl<T, Err> Outgoing<T, Err>
where
    Err: Error,
{
    pub async fn send(&mut self, v: Result<T, Err>) -> Result<(), Err> {
        SinkExt::send(&mut self.inner, v)
            .map(|x| match x {
                Ok(()) => Ok(()),
                Result::Err(e) => Result::Err(Error::custom(e)),
            })
            .await
    }
}

impl<T, Err> Clone for Outgoing<T, Err> {
    fn clone(&self) -> Self {
        Outgoing {
            inner: self.inner.clone(),
        }
    }
}
