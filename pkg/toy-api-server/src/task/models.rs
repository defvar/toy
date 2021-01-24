use core::time::Duration;
use toy::core::prelude::{TaskId, Value};
use toy::supervisor::TaskResponse;
use toy_pack::Pack;

#[derive(Debug, Pack)]
pub struct RunningTasksEntity {
    tasks: Vec<RunningTasksInner>,
    count: u32,
}

#[derive(Debug, Pack)]
struct RunningTasksInner {
    task_id: TaskId,
    started_at: Duration,
    graph: Value,
}

impl RunningTasksEntity {
    pub fn from(r: Vec<TaskResponse>) -> Self {
        let tasks = r
            .iter()
            .map(|x| RunningTasksInner {
                task_id: x.id(),
                started_at: x.started_at(),
                graph: x.graph().original(),
            })
            .collect::<Vec<_>>();
        let count = tasks.len() as u32;
        Self { tasks, count }
    }
}
