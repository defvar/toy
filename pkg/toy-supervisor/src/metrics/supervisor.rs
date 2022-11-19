use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use toy_core::metrics::{Counter, TaskMetrics};
use toy_core::task::TaskId;

#[derive(Debug, Clone)]
pub struct SupervisorMetrics {
    task_start_count: Counter,
    task_metrics: Arc<Mutex<HashMap<TaskId, Arc<TaskMetrics>>>>,
}

impl SupervisorMetrics {
    pub fn new() -> Self {
        Self {
            task_start_count: Counter::from(0),
            task_metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn task_start_count(&self) -> u64 {
        self.task_start_count.get().unwrap_or(0)
    }

    pub fn inc_task_start_count(&self) {
        self.task_start_count.increment()
    }

    pub async fn new_task_metrics(&mut self, id: TaskId) -> Arc<TaskMetrics> {
        let mut lock = self.task_metrics.lock().await;
        lock.insert(id, Arc::new(TaskMetrics::new()));
        lock.get(&id).map(|x| Arc::clone(&x)).unwrap()
    }

    pub async fn task_metrics(&self) -> Vec<(TaskId, TaskMetrics)> {
        self.task_metrics
            .lock()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), (**v).clone()))
            .collect()
    }
}
