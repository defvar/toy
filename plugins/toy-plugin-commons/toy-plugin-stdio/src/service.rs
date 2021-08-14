use crate::config::{StdinConfig, StdoutConfig};
use tokio::io::AsyncWriteExt;
use tokio::io::Stdin;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use toy_core::prelude::*;

#[allow(dead_code)]
pub struct StdinContext {
    config: StdinConfig,
    reader: ReaderStream<Stdin>,
}

#[allow(dead_code)]
pub struct StdoutContext {
    config: StdoutConfig,
    writer: tokio::io::Stdout,
}

pub fn new_stdin_context(
    _tp: ServiceType,
    config: StdinConfig,
) -> Result<StdinContext, ServiceError> {
    let reader = ReaderStream::new(tokio::io::stdin());
    Ok(StdinContext { config, reader })
}

pub fn new_stdout_context(
    _tp: ServiceType,
    config: StdoutConfig,
) -> Result<StdoutContext, ServiceError> {
    let writer = tokio::io::stdout();
    Ok(StdoutContext { config, writer })
}

pub async fn stdin(
    _task_ctx: TaskContext,
    mut ctx: StdinContext,
    _req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<StdinContext>, ServiceError> {
    let v = ctx.reader.next().await;
    match v {
        Some(Ok(bytes)) => {
            tx.send_ok(Frame::from(&bytes[..])).await?;
        }
        Some(Err(e)) => {
            return Err(ServiceError::error(e));
        }
        None => {}
    }
    Ok(ServiceContext::Ready(ctx))
}

pub async fn stdout(
    _task_ctx: TaskContext,
    mut ctx: StdoutContext,
    req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<StdoutContext>, ServiceError> {
    match req.value() {
        Some(v) => match v.parse_str() {
            Some(str) => {
                ctx.writer.write_all(str.as_bytes()).await?;
                ctx.writer.write(&[b'\r', b'\n']).await?;
            }
            None => (),
        },
        None => (),
    };
    tx.send(Ok(Frame::none())).await?;
    Ok(ServiceContext::Ready(ctx))
}
