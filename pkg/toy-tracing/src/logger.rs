use crate::tcp::TcpLogger;
use std::net::ToSocketAddrs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

pub fn console() {
    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    let builder = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time);
    builder.init();
}

pub fn tcp<A: ToSocketAddrs>(addr: A) -> Result<WorkerGuard, std::io::Error> {
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
    Ok(guard)
}
