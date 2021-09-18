use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supervisor {
    name: String,
    start_time: String,
    labels: Vec<String>,
}

impl Supervisor {
    pub fn new(name: String, start_time: String, labels: Vec<String>) -> Self {
        Self {
            name,
            start_time,
            labels,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupervisorList {
    items: Vec<Supervisor>,
    count: u32,
}

impl SupervisorList {
    pub fn new(items: Vec<Supervisor>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}
