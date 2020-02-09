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
pub enum ServiceError {
    #[fail(display = "channel canceled")]
    ChannelCanceled,

    #[fail(display = "channel send error: {:?}", inner)]
    ChannelSendError { inner: SendError },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl ServiceError {
    pub fn error<T>(msg: T) -> ServiceError
    where
        T: Display,
    {
        ServiceError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<oneshot::Canceled> for ServiceError {
    fn from(_e: oneshot::Canceled) -> ServiceError {
        ServiceError::ChannelCanceled
    }
}

impl From<SendError> for ServiceError {
    fn from(e: SendError) -> Self {
        ServiceError::ChannelSendError { inner: e }
    }
}

impl<T> From<TrySendError<T>> for ServiceError {
    fn from(e: TrySendError<T>) -> Self {
        ServiceError::ChannelSendError {
            inner: e.into_send_error(),
        }
    }
}

impl Error for ServiceError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ServiceError::Error {
            inner: msg.to_string(),
        }
    }
}
