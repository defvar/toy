#![feature(backtrace)]

mod client;
mod error;

pub use client::Client;
pub use error::RocksError;
