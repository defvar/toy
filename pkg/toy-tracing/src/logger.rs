use crate::tcp::TcpLogger;
use crate::{LogFormat, LogOption};
use std::net::ToSocketAddrs;
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
pub struct LogGuard {
    _g: Option<WorkerGuard>,
}

/// enable console log.
pub fn console() -> Result<LogGuard, std::io::Error> {
    console_with(LogOption::default())
}

/// enable console log.
pub fn console_with(opt: LogOption) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder()
        .server_addr(opt.tokio_console_addr)
        .spawn();

    let (t, j) = match opt.format {
        LogFormat::Text => (
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(opt.ansi)
                    .with_timer(time)
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_filter(EnvFilter::from_default_env()),
            ),
            None,
        ),
        LogFormat::Json => (
            None,
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(opt.ansi)
                    .with_timer(time)
                    .json()
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_filter(EnvFilter::from_default_env()),
            ),
        ),
    };

    tracing_subscriber::registry()
        .with(tokio_console_layer)
        .with(t)
        .with(j)
        .init();
    Ok(LogGuard { _g: None })
}

pub fn file(dir: impl AsRef<Path>, prefix: impl AsRef<Path>) -> Result<LogGuard, std::io::Error> {
    file_with(dir, prefix, LogOption::default())
}

pub fn file_with(
    dir: impl AsRef<Path>,
    prefix: impl AsRef<Path>,
    opt: LogOption,
) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder()
        .server_addr(opt.tokio_console_addr)
        .spawn();

    let fw_log_dir = dir.as_ref().to_path_buf();
    let fw_log_prefix = format!("{}.{}", prefix.as_ref().to_str().unwrap(), "fw");

    let toy_appender = opt.rotation.to_rolling(dir, prefix);

    let fw_appender = opt
        .rotation
        .to_rolling(fw_log_dir, fw_log_prefix)
        .with_filter(|meta| {
            !meta.target().starts_with("toy") && !meta.target().starts_with("actor")
        });

    let appender = toy_appender.and(fw_appender);

    let (t, j) = match opt.format {
        LogFormat::Text => (
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(opt.ansi)
                    .with_timer(time.clone())
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_writer(std::io::stdout),
            ),
            None,
        ),
        LogFormat::Json => (
            None,
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(opt.ansi)
                    .with_timer(time.clone())
                    .json()
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_writer(std::io::stdout),
            ),
        ),
    };

    let (t_file, j_file) = match opt.format {
        LogFormat::Text => (
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_timer(time)
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_writer(appender),
            ),
            None,
        ),
        LogFormat::Json => (
            None,
            Some(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_timer(time)
                    .json()
                    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_writer(appender),
            ),
        ),
    };

    //let (non_blocking, guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tokio_console_layer)
        .with(EnvFilter::from_default_env())
        .with(t)
        .with(j)
        .with(t_file)
        .with(j_file)
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
