use core::time::Duration;
use toy::core::prelude::{TaskId, Value};
use toy::supervisor::{RunTaskResponse, TaskResponse};
use toy_pack::Pack;

#[derive(Debug, Pack)]
pub struct RunTaskEntity {
    task_id: TaskId,
}

impl RunTaskEntity {
    pub fn from(r: RunTaskResponse) -> Self {
        Self { task_id: r.id() }
    }
}

#[derive(Debug, Pack)]
pub struct ListTaskEntity {
    tasks: Vec<Inner>,
    count: u32,
}

#[derive(Debug, Pack)]
struct Inner {
    task_id: TaskId,
    started_at: Duration,
    graph: Value,
}

impl ListTaskEntity {
    pub fn from(r: Vec<TaskResponse>) -> Self {
        let tasks = r
            .iter()
            .map(|x| Inner {
                task_id: x.id(),
                started_at: x.started_at(),
                graph: x.graph().original(),
            })
            .collect::<Vec<_>>();
        let count = tasks.len() as u32;
        Self { tasks, count }
    }
}
