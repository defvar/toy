pub mod auth;
pub mod codec;
pub mod error;
pub mod request;

#[cfg(feature = "server")]
pub mod reply;

pub use error::Error;

#[doc(hidden)]
pub use toy_h::bytes;

#[cfg(feature = "server")]
#[doc(hidden)]
pub use axum;

#[cfg(feature = "server")]
#[doc(hidden)]
pub use axum_server;
