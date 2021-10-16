#![feature(type_alias_impl_trait)]

mod plugin;
mod write;

pub mod config {
    pub use super::write::TcpWriteConfig;
}

pub mod service {
    pub use super::write::{TcpWrite, TcpWriteContext};
}

pub use plugin::{all, write};
