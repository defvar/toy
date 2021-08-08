use crate::{Handler, TailConfig};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct FlushTimer {
    last_handle_at: Arc<Mutex<Option<Instant>>>,
    prev_handle_at: Option<Instant>,
    check_interval_millis: u64,
    threshold_millis: u64,
    handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
}

impl FlushTimer {
    pub fn new(
        handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
        last_handle_at: Arc<Mutex<Option<Instant>>>,
        config: &TailConfig,
    ) -> Self {
        Self {
            handlers,
            last_handle_at,
            prev_handle_at: None,
            check_interval_millis: config.check_interval_millis(),
            threshold_millis: config.threshold_millis(),
        }
    }

    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_millis(self.check_interval_millis));

        loop {
            interval.tick().await;

            let now = Instant::now();

            let handle_at = self.last_handle_at.lock().await;
            if self.prev_handle_at == *handle_at {
                continue;
            }

            match *handle_at {
                Some(last)
                    if now.duration_since(last) > Duration::from_millis(self.threshold_millis) =>
                {
                    let mut handler = self.handlers.lock().await;
                    self.prev_handle_at = Some(last);
                    for h in handler.iter_mut() {
                        let r = h.flush().await;
                        if let Err(e) = r {
                            tracing::error!("error flush timer. error:{:?}", e);
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
