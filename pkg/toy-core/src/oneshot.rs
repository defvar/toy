use crate::error::OutgoingError;
use tokio::sync::oneshot::{self, Receiver, Sender};

pub fn channel<T>() -> (Outgoing<T>, Incoming<T>) {
    let (tx, rx) = oneshot::channel();
    (Outgoing::new(tx), Incoming::new(rx))
}

#[derive(Debug)]
pub struct Incoming<T> {
    inner: Receiver<T>,
}

impl<T> Incoming<T> {
    pub fn new(rx: Receiver<T>) -> Incoming<T> {
        Incoming { inner: rx }
    }

    pub async fn recv(self) -> Option<T> {
        self.inner.await.ok()
    }
}

#[derive(Debug)]
pub struct Outgoing<T> {
    inner: Sender<T>,
}

impl<T> Outgoing<T> {
    pub fn new(tx: Sender<T>) -> Outgoing<T> {
        Outgoing { inner: tx }
    }
}

impl<T> Outgoing<T> {
    pub async fn send(self, v: T) -> Result<(), OutgoingError> {
        if let Err(_) = self.inner.send(v) {
            Err(OutgoingError::receiver_dropped())
        } else {
            Ok(())
        }
    }
}
