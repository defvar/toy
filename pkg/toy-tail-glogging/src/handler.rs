use async_trait::async_trait;
use toy_glogging::client::Client;
use toy_glogging::error::GLoggingError;
use toy_glogging::models::{Entry, EntryBuilder, Resource, Severity, WriteRequest, WriteResponse};
use toy_glogging::reqwest;
use toy_tail::{Flagments, Handler, TailError};

pub struct GLoggingHandler {
    client: Client,
    buffer_size: usize,
    buffer: Option<Vec<Entry>>,
}

impl GLoggingHandler {
    pub fn from(c: reqwest::Client, buffer_size: usize) -> GLoggingHandler {
        Self {
            client: Client::from(c),
            buffer_size,
            buffer: Some(Vec::with_capacity(buffer_size)),
        }
    }

    async fn flush_if_needed(&mut self, force: bool) -> Result<(), GLoggingError> {
        match self.buffer {
            Some(ref v) if v.len() > 0 && (force || v.len() >= self.buffer_size) => {
                let buf = self.buffer.replace(Vec::with_capacity(self.buffer_size));
                let c = self.client.clone();
                flush(c, buf.unwrap()).await.map(|_| ())
            }
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl Handler for GLoggingHandler {
    async fn flagments(&mut self, fl: Flagments<'_>) -> Result<(), TailError> {
        let e = EntryBuilder::new(
            "projects/crested-sunup-195408/logs/toy",
            Resource::new("global"),
        )
        .severity(Severity::INFO)
        .kv("task_id", fl.task_id().unwrap_or_else(|| ""))
        .kv("message", fl.message().unwrap_or_else(|| ""))
        .label("task_id", fl.task_id().unwrap_or_else(|| ""))
        .build();
        self.buffer.as_mut().map(|x| x.push(e));
        self.flush_if_needed(false)
            .await
            .map_err(|x| TailError::error(x))
    }

    async fn flush(&mut self) -> Result<(), TailError> {
        self.flush_if_needed(true)
            .await
            .map_err(|x| TailError::error(x))
    }
}

async fn flush(client: Client, entries: Vec<Entry>) -> Result<WriteResponse, GLoggingError> {
    let token =
        toy_glogging::auth::request_token(client.raw(), toy_glogging::auth::Scope::LoggingWrite)
            .await?;
    let req = WriteRequest::from_entries(entries);
    client.write(token, req).await
}
