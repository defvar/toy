use crate::flush_timer::FlushTimer;
use crate::{Flagments, TailConfig, TailError};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use toy_text_parser::Line;

#[async_trait]
pub trait Handler: Send {
    /// handler name for logging.
    fn name(&self) -> &'static str;

    /// Called after parsed one line.
    /// If buffering is required, it must be done on the implementation side.
    async fn flagments(&mut self, fl: Flagments<'_>) -> Result<(), TailError>;

    /// Called after parsed one line.
    async fn raw(&mut self, raw: &'_ Line, parse_successed: bool) -> Result<(), TailError> {
        let _ = raw;
        let _ = parse_successed;
        Ok(())
    }

    /// Called when elapsed threshold by `FlushTimer`.
    async fn flush(&mut self) -> Result<(), TailError>;
}

pub struct Handlers {
    last_handle_at: Arc<Mutex<Option<Instant>>>,
    handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
}

impl Handlers {
    pub fn new(handlers: Vec<Box<dyn Handler>>, config: &TailConfig) -> (Handlers, FlushTimer) {
        let handlers = Arc::new(Mutex::new(handlers));
        let last_handle_at = Arc::new(Mutex::new(None));
        (
            Self {
                handlers: Arc::clone(&handlers),
                last_handle_at: Arc::clone(&last_handle_at),
            },
            FlushTimer::new(Arc::clone(&handlers), Arc::clone(&last_handle_at), config),
        )
    }

    pub async fn handle(&self, fl: Flagments<'_>, line: &Line) -> Result<(), TailError> {
        let mut handle_at = self.last_handle_at.lock().await;
        let mut handlers = self.handlers.lock().await;
        let now = Instant::now();
        let parsed = fl.is_some();

        if handlers.len() == 1 {
            let h = handlers.get_mut(0).unwrap();
            h.flagments(fl).await?;
            h.raw(&line, parsed).await?;
        } else {
            for h in handlers.iter_mut() {
                h.flagments(fl.clone()).await?;
                h.raw(&line, parsed).await?;
            }
        }

        *handle_at = Some(now);

        Ok(())
    }

    pub async fn handler_names(&self) -> Vec<String> {
        let mut buf = Vec::new();
        {
            let handlers = self.handlers.lock().await;
            for h in handlers.iter() {
                buf.push(h.name().to_string());
            }
        }
        buf
    }
}
