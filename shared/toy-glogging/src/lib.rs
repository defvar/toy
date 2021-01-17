#![feature(backtrace)]

mod client;
pub mod error;
pub mod models;

pub use client::Client;

pub mod auth {
    #[doc(hidden)]
    pub use toy_gauth::{error::GAuthError, request_token, GToken, Scope};
}
