use crate::error::Error;
use tokio::sync::mpsc::{self, Receiver, Sender};

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
        self.inner.recv().await
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
        if let Result::Err(e) = self.inner.send(v).await {
            Result::Err(Error::custom(e))
        } else {
            Ok(())
        }
    }
}

impl<T, Err> Clone for Outgoing<T, Err> {
    fn clone(&self) -> Self {
        Outgoing {
            inner: self.inner.clone(),
        }
    }
}
