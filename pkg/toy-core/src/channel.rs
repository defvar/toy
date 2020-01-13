use crate::error::MessagingError;
use futures::channel::mpsc::{self, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::Future;

pub fn stream<T>(buffer: usize) -> (Outgoing<T>, Incoming<T>) {
    let (tx, rx) = mpsc::channel(buffer);
    (Outgoing::new(tx), Incoming::new(rx))
}

pub struct Incoming<T> {
    inner: Receiver<T>,
}

impl<T> Incoming<T> {
    pub fn new(rx: Receiver<T>) -> Incoming<T> {
        Incoming { inner: rx }
    }

    pub async fn next(&mut self) -> Option<T> {
        StreamExt::next(&mut self.inner).await
    }

    pub async fn for_each<Fut, F>(self, f: F) -> ()
    where
        F: FnMut(T) -> Fut,
        Fut: Future<Output = ()>,
        Self: Sized,
    {
        StreamExt::for_each(self.inner, f).await
    }
}

pub struct Outgoing<T> {
    inner: Sender<T>,
}

impl<T> Outgoing<T> {
    pub fn new(tx: Sender<T>) -> Outgoing<T> {
        Outgoing { inner: tx }
    }

    pub async fn send(&mut self, v: T) -> Result<(), MessagingError> {
        SinkExt::send(&mut self.inner, v)
            .await
            .map_err(|e| e.into())
    }
}
