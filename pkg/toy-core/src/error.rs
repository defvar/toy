use std::fmt::Display;

use failure::Fail;
use futures::channel::mpsc::{SendError, TrySendError};
use futures::channel::oneshot;

pub trait Error: Sized + Fail {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}

#[derive(Debug, Fail)]
pub enum MessagingError {
    #[fail(display = "channel canceled")]
    ChannelCanceled,

    #[fail(display = "channel send error: {:?}", inner)]
    ChannelSendError { inner: SendError },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl MessagingError {
    pub fn error<T>(msg: T) -> MessagingError
    where
        T: Display,
    {
        MessagingError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<oneshot::Canceled> for MessagingError {
    fn from(_e: oneshot::Canceled) -> MessagingError {
        MessagingError::ChannelCanceled
    }
}

impl From<SendError> for MessagingError {
    fn from(e: SendError) -> Self {
        MessagingError::ChannelSendError { inner: e }
    }
}

impl<T> From<TrySendError<T>> for MessagingError {
    fn from(e: TrySendError<T>) -> Self {
        MessagingError::ChannelSendError {
            inner: e.into_send_error(),
        }
    }
}

impl Error for MessagingError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        MessagingError::Error {
            inner: msg.to_string(),
        }
    }
}
