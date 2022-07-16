use crate::tcp::TcpLogger;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
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
/// /// toki-tracing using default addr.
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
    file_with_addr(
        dir,
        prefix,
        rotaion,
        SocketAddr::new(CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT),
    )
}

pub fn file_with_addr(
    dir: impl AsRef<Path>,
    prefix: impl AsRef<Path>,
    rotaion: LogRotation,
    addr: impl Into<SocketAddr>,
) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder()
        .server_addr(addr)
        .spawn();

    let fw_log_dir = dir.as_ref().to_path_buf();
    let fw_log_prefix = format!("{}.{}", prefix.as_ref().to_str().unwrap(), "fw");

    let toy_appender = match rotaion {
        LogRotation::Minutely => rolling::minutely(dir, prefix),
        LogRotation::Hourly => rolling::hourly(dir, prefix),
        LogRotation::Daily => rolling::daily(dir, prefix),
        LogRotation::Never => rolling::never(dir, prefix),
    };

    let fw_appender = match rotaion {
        LogRotation::Minutely => rolling::minutely(fw_log_dir, fw_log_prefix),
        LogRotation::Hourly => rolling::hourly(fw_log_dir, fw_log_prefix),
        LogRotation::Daily => rolling::daily(fw_log_dir, fw_log_prefix),
        LogRotation::Never => rolling::never(fw_log_dir, fw_log_prefix),
    }
    .with_filter(|meta| {
        !meta.target().starts_with("toy") && !meta.target().starts_with("supervisor")
    });

    let appender = toy_appender.and(fw_appender);

    //let (non_blocking, guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tokio_console_layer)
        .with(EnvFilter::from_default_env())
        .with(
            fmt::Layer::new()
                .with_timer(time.clone())
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_ansi(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_writer(std::io::stdout),
        )
        .with(
            fmt::Layer::new()
                .with_timer(time)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_ansi(false)
                .json()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_writer(appender),
        )
        .init();

    //Ok(LogGuard { _g: Some(guard) })
    Ok(LogGuard { _g: None })
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
