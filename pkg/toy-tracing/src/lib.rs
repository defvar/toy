mod logger;
pub mod tcp;

pub use logger::{console, file, tcp, LogGuard, LogRotation};