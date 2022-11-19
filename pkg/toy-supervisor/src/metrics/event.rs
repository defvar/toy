use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use toy_core::metrics::{EventRecord, Events};
use toy_core::task::TaskId;

#[derive(Clone, Debug)]
pub struct EventCache {
    task_events: Arc<Mutex<HashMap<TaskId, Arc<Mutex<Events>>>>>,
}

impl EventCache {
    pub fn new() -> EventCache {
        Self {
            task_events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn new_task_events(&mut self, id: TaskId) -> Arc<Mutex<Events>> {
        let mut lock = self.task_events.lock().await;
        lock.insert(id, Arc::new(Mutex::new(Events::new())));
        lock.get(&id).map(|x| Arc::clone(x)).unwrap()
    }

    pub async fn records(&self) -> Vec<EventRecord> {
        let mut r = Vec::new();
        for item in self.task_events.lock().await.iter() {
            {
                let lock = item.1.lock().await;
                r.extend(lock.records());
            }
        }
        r
    }
}
