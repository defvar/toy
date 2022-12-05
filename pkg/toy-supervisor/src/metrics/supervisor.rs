use toy_core::metrics::Counter;

#[derive(Debug, Clone)]
pub struct SupervisorMetrics {
    task_start_count: Counter,
}

impl SupervisorMetrics {
    pub fn new() -> Self {
        Self {
            task_start_count: Counter::from(0),
        }
    }

    pub fn task_start_count(&self) -> u64 {
        self.task_start_count.get().unwrap_or(0)
    }

    pub fn inc_task_start_count(&self) {
        self.task_start_count.increment()
    }
}
