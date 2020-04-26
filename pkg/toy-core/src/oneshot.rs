use crate::error::Error;
use tokio::sync::oneshot::{self, Receiver, Sender};

pub fn channel<T, Err>() -> (Outgoing<T, Err>, Incoming<T, Err>) {
    let (tx, rx) = oneshot::channel();
    (Outgoing::new(tx), Incoming::new(rx))
}

#[derive(Debug)]
pub struct Incoming<T, Err> {
    inner: Receiver<Result<T, Err>>,
}

impl<T, Err> Incoming<T, Err> {
    pub fn new(rx: Receiver<Result<T, Err>>) -> Incoming<T, Err> {
        Incoming { inner: rx }
    }

    pub async fn recv(self) -> Option<Result<T, Err>> {
        self.inner.await.ok()
    }
}

#[derive(Debug)]
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
    pub async fn send(self, v: Result<T, Err>) -> Result<(), Err> {
        if let Result::Err(_) = self.inner.send(v) {
            Result::Err(Error::custom("the receiver dropped"))
        } else {
            Ok(())
        }
    }

    pub async fn send_ok(self, v: T) -> Result<(), Err> {
        self.send(Ok(v)).await
    }
}
