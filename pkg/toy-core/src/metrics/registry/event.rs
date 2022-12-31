use crate::metrics::{EventRecord, MetricsEvents};
use crate::task::TaskId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EventRegistry {
    events: Arc<Mutex<HashMap<TaskId, Arc<Mutex<MetricsEvents>>>>>,
}

impl EventRegistry {
    pub fn new() -> EventRegistry {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn new_task_events(&self, id: TaskId) -> Arc<Mutex<MetricsEvents>> {
        let mut lock = self.events.lock().await;
        lock.insert(id, Arc::new(Mutex::new(MetricsEvents::new())));
        lock.get(&id).map(|x| Arc::clone(x)).unwrap()
    }

    pub async fn get_or_create(&self, id: TaskId) -> Arc<Mutex<MetricsEvents>> {
        let mut lock = self.events.lock().await;
        if let Some(entry) = lock.get(&id) {
            Arc::clone(entry)
        } else {
            lock.insert(id, Arc::new(Mutex::new(MetricsEvents::new())));
            lock.get(&id).map(|x| Arc::clone(x)).unwrap()
        }
    }

    pub async fn records(&self) -> Vec<EventRecord> {
        let mut r = Vec::new();
        for item in self.events.lock().await.iter() {
            {
                let lock = item.1.lock().await;
                r.extend(lock.records());
            }
        }
        r
    }

    pub async fn drain(&self) -> Vec<EventRecord> {
        let mut r = Vec::new();
        for (_, v) in self.events.lock().await.drain() {
            {
                let lock = v.lock().await;
                r.extend(lock.records());
            }
        }
        r
    }
}
