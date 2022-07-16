mod logger;
pub mod tcp;

pub use logger::{console, console_with, file, file_with, tcp, LogGuard};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use tracing_appender::rolling::{self, RollingFileAppender};

pub const CONSOLE_DEFAULT_IP: IpAddr = console_subscriber::Server::DEFAULT_IP;

pub const CONSOLE_DEFAULT_PORT: u16 = console_subscriber::Server::DEFAULT_PORT;

#[derive(Debug, Clone, Copy)]
pub enum LogRotation {
    Minutely,
    Hourly,
    Daily,
    Never,
}

impl LogRotation {
    pub(crate) fn to_rolling(
        &self,
        directory: impl AsRef<Path>,
        file_name_prefix: impl AsRef<Path>,
    ) -> RollingFileAppender {
        match &self {
            LogRotation::Minutely => rolling::minutely(directory, file_name_prefix),
            LogRotation::Hourly => rolling::hourly(directory, file_name_prefix),
            LogRotation::Daily => rolling::daily(directory, file_name_prefix),
            LogRotation::Never => rolling::never(directory, file_name_prefix),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Debug, Clone)]
pub struct LogOption {
    format: LogFormat,
    ansi: bool,
    tokio_console_addr: SocketAddr,
    rotation: LogRotation,
}

impl LogOption {
    pub fn with_format(self, format: LogFormat) -> Self {
        Self { format, ..self }
    }

    pub fn with_ansi(self, ansi: bool) -> Self {
        Self { ansi, ..self }
    }

    pub fn with_tokio_console_addr(self, tokio_console_addr: SocketAddr) -> Self {
        Self {
            tokio_console_addr,
            ..self
        }
    }

    pub fn with_rotation(self, rotation: LogRotation) -> Self {
        Self { rotation, ..self }
    }
}

impl Default for LogOption {
    fn default() -> Self {
        LogOption {
            format: LogFormat::Text,
            ansi: false,
            tokio_console_addr: SocketAddr::new(CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT),
            rotation: LogRotation::Never,
        }
    }
}
