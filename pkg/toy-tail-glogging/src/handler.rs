use async_trait::async_trait;
use toy_glogging::error::GLoggingError;
use toy_glogging::models::{
    Entry, EntryBuilder, Operation, Resource, Severity, WriteRequest, WriteResponse,
};
use toy_glogging::Client;
use toy_h::HttpClient;
use toy_tail::{Flagments, Handler, TailError};

pub struct GLoggingHandler<T> {
    client: Client<T>,
    buffer_size: usize,
    buffer: Option<Vec<Entry>>,
    log_name: String,
}

impl<T> GLoggingHandler<T>
where
    T: HttpClient,
{
    pub fn from(c: T, log_name: String, buffer_size: usize) -> GLoggingHandler<T> {
        Self {
            client: Client::from(c),
            buffer_size,
            buffer: Some(Vec::with_capacity(buffer_size)),
            log_name,
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
impl<T> Handler for GLoggingHandler<T>
where
    T: HttpClient,
{
    fn name(&self) -> &'static str {
        "GLogging"
    }

    async fn flagments(&mut self, fl: Flagments<'_>) -> Result<(), TailError> {
        let ope = match (fl.graph(), fl.uri(), fl.node_busy_time()) {
            (Some(_), None, None) => Some(Operation::first(fl.task_id().unwrap_or_else(|| ""))),
            (Some(_), None, Some(_)) => Some(Operation::last(fl.task_id().unwrap_or_else(|| ""))),
            _ => None,
        };

        let e = EntryBuilder::new(&self.log_name, Resource::new("global"))
            .severity(Severity::INFO)
            .timestamp(fl.datetime().unwrap_or_else(|| ""))
            .kv_opt("task_id", fl.task_id())
            .kv_opt("graph", fl.graph())
            .kv_opt("message", fl.message())
            .kv_opt("target", fl.target())
            .kv_opt("uri", fl.uri())
            .kv_opt("busy", fl.node_busy_time())
            .kv_opt("idle", fl.node_idle_time())
            .label_opt("task_id", fl.task_id())
            .label_opt("graph", fl.graph())
            .label_opt("uri", fl.uri())
            .opelation_opt(ope)
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

async fn flush<T>(client: Client<T>, entries: Vec<Entry>) -> Result<WriteResponse, GLoggingError>
where
    T: HttpClient,
{
    let token =
        toy_glogging::auth::request_token(client.raw(), toy_glogging::auth::Scope::LoggingWrite)
            .await?;
    let req = WriteRequest::from_entries(entries);
    client.write(token, req).await
}
