use crate::tcp::TcpLogger;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub enum LogRotation {
    Minutely,
    Hourly,
    Daily,
    Never,
}

#[derive(Debug)]
pub struct LogGuard {
    _g: Option<WorkerGuard>,
}

pub const CONSOLE_DEFAULT_IP: IpAddr = console_subscriber::Server::DEFAULT_IP;
pub const CONSOLE_DEFAULT_PORT: u16 = console_subscriber::Server::DEFAULT_PORT;

/// enable console log.
pub fn console() -> Result<LogGuard, std::io::Error> {
    console_with_addr(SocketAddr::new(CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT))
}

/// enable console log.
/// toki-tracing using addr.
pub fn console_with_addr(addr: impl Into<SocketAddr>) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder()
        .server_addr(addr)
        .spawn();

    tracing_subscriber::registry()
        .with(tokio_console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(time)
                .json()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_filter(EnvFilter::from_default_env()),
        )
        .init();
    Ok(LogGuard { _g: None })
}

pub fn file(
    dir: impl AsRef<Path>,
    prefix: impl AsRef<Path>,
    rotaion: LogRotation,
) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let file_appender = match rotaion {
        LogRotation::Minutely => tracing_appender::rolling::minutely(dir, prefix),
        LogRotation::Hourly => tracing_appender::rolling::hourly(dir, prefix),
        LogRotation::Daily => tracing_appender::rolling::daily(dir, prefix),
        LogRotation::Never => tracing_appender::rolling::never(dir, prefix),
    };
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let builder = tracing_subscriber::fmt()
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time)
        .with_ansi(false)
        .with_writer(non_blocking);

    builder.init();
    Ok(LogGuard { _g: Some(guard) })
}

pub fn tcp<A: ToSocketAddrs>(addr: A) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let tcp = TcpLogger::new(addr)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(tcp);
    let builder = tracing_subscriber::fmt()
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time)
        .with_ansi(false)
        .with_writer(non_blocking);

    builder.init();
    Ok(LogGuard { _g: Some(guard) })
}
