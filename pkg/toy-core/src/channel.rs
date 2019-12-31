use core::pin::Pin;
use core::task::{Context, Poll};
use futures::channel::mpsc::{self, Receiver, Sender};
use futures::channel::oneshot;
use futures::future::{poll_fn, Future, FutureExt};

use super::data::Frame;
use super::error::MessagingError;

pub fn oneshot() -> (impl Outgoing<Output = Frame>, OneshotIncoming<Frame>) {
    let (tx, rx) = oneshot::channel::<Frame>();
    (tx, OneshotIncoming::new(rx))
}

pub fn stream<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    mpsc::channel(buffer)
}

pub trait Outgoing {
    type Output;

    fn is_canceled(&self) -> bool;

    fn send(self, r: Self::Output);
}

impl Outgoing for oneshot::Sender<Frame> {
    type Output = Frame;

    fn is_canceled(&self) -> bool {
        oneshot::Sender::is_canceled(self)
    }

    fn send(self, r: Self::Output) {
        let _ = Self::send(self, r);
    }
}

pub struct OneshotIncoming<T> {
    inner: oneshot::Receiver<T>,
}

impl<T> Unpin for OneshotIncoming<T> {}

impl<T> OneshotIncoming<T> {
    pub fn new(receiver: oneshot::Receiver<T>) -> OneshotIncoming<T> {
        OneshotIncoming { inner: receiver }
    }

    pub async fn on<F>(&mut self, f: F) -> Result<T, MessagingError>
    where
        F: FnOnce(Result<T, MessagingError>) -> Result<T, MessagingError>,
        F: Send,
    {
        poll_fn(|cx| self.poll_unpin(cx)).map(f).await
    }
}

impl<T> Future for OneshotIncoming<T> {
    type Output = Result<T, MessagingError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<T, MessagingError>> {
        self.inner.poll_unpin(cx).map(|x| match x {
            Ok(r) => Ok(r),
            Err(_) => Err(MessagingError::ChannelCanceled),
        })
    }
}
