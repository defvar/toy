use crate::tcp::TcpLogger;
use std::net::ToSocketAddrs;
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
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

pub fn console() -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    let builder = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time);
    builder.init();
    Ok(LogGuard { _g: None })
}

pub fn file(
    dir: impl AsRef<Path>,
    prefix: impl AsRef<Path>,
    rotaion: LogRotation,
) -> Result<LogGuard, std::io::Error> {
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    let file_appender = match rotaion {
        LogRotation::Minutely => tracing_appender::rolling::minutely(dir, prefix),
        LogRotation::Hourly => tracing_appender::rolling::hourly(dir, prefix),
        LogRotation::Daily => tracing_appender::rolling::daily(dir, prefix),
        LogRotation::Never => tracing_appender::rolling::never(dir, prefix),
    };
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let builder = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
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
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    let tcp = TcpLogger::new(addr)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(tcp);
    let builder = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time)
        .with_ansi(false)
        .with_writer(non_blocking);

    builder.init();
    Ok(LogGuard { _g: Some(guard) })
}
