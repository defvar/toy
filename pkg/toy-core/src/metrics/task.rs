use crate::metrics::Counter;
use serde::Serialize;

/// Metrics for task.
#[derive(Debug, Clone, Serialize)]
pub struct TaskMetrics {
    service_start_count: Counter,
    service_finish_count: Counter,
    request_send_count: Counter,
    request_receive_count: Counter,
    request_complete_count: Counter,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            service_start_count: Counter::from(0),
            service_finish_count: Counter::from(0),
            request_send_count: Counter::from(0),
            request_receive_count: Counter::from(0),
            request_complete_count: Counter::from(0),
        }
    }

    pub fn inc_service_start_count(&self) {
        self.service_start_count.increment()
    }

    pub fn inc_service_finish_count(&self) {
        self.service_finish_count.increment()
    }

    pub fn inc_request_send_count(&self) {
        self.request_send_count.increment()
    }

    pub fn inc_request_receive_count(&self) {
        self.request_receive_count.increment()
    }

    pub fn inc_request_complete_count(&self) {
        self.request_complete_count.increment()
    }
}
