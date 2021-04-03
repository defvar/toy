use crate::graph::GraphEntity;
use toy_core::prelude::TaskId;
use toy_pack::{Pack, Unpack};

#[derive(Debug, Clone, Pack, Unpack)]
pub struct PendingEntity {
    graph: Option<GraphEntity>,
}

#[derive(Debug, Clone, Pack, Unpack)]
pub struct PendingsEntity {
    pendings: Vec<PendingEntity>,
    count: u32,
}

#[derive(Debug, Clone, Pack)]
pub struct PendingResult {
    task_id: TaskId,
}

#[derive(Debug, Pack, Unpack)]
pub struct TaskLogEntity {
    task_id: TaskId,
    payload: Vec<TaskLogInner>,
    count: u32,
}

#[derive(Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct TaskLogInner {
    message: String,
    target: String,
    graph: String,
    uri: Option<String>,
    busy: Option<String>,
    idle: Option<String>,
}

#[derive(Debug, Pack, Unpack)]
pub struct TasksEntity {
    tasks: Vec<TasksInner>,
    count: u32,
}

#[derive(Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct TasksInner {
    task_id: TaskId,
    started_at: Option<String>,
    ended_at: Option<String>,
}

impl PendingEntity {
    pub fn new(graph: GraphEntity) -> Self {
        Self { graph: Some(graph) }
    }

    pub fn graph(&self) -> Option<&GraphEntity> {
        self.graph.as_ref()
    }
}

impl PendingsEntity {
    pub fn new(pendings: Vec<PendingEntity>) -> Self {
        let count = pendings.len() as u32;
        Self { pendings, count }
    }

    pub fn pendings(&self) -> &Vec<PendingEntity> {
        &self.pendings
    }
}

impl PendingResult {
    pub fn from_id(id: TaskId) -> Self {
        Self { task_id: id }
    }
}

impl TaskLogEntity {
    pub fn new(task_id: TaskId, payload: Vec<TaskLogInner>) -> Self {
        let count = payload.len() as u32;
        Self {
            task_id,
            payload,
            count,
        }
    }
}

impl TasksEntity {
    pub fn new(tasks: Vec<TasksInner>) -> Self {
        let count = tasks.len() as u32;
        Self { tasks, count }
    }
}

impl TasksInner {
    pub fn new<T: AsRef<str>>(task_id: T) -> Result<Self, ()> {
        let id = match TaskId::parse_str(task_id.as_ref()) {
            Ok(id) => id,
            Err(_) => return Err(()),
        };
        Ok(Self {
            task_id: id,
            started_at: None,
            ended_at: None,
        })
    }

    pub fn with_started_at<T: AsRef<str>>(self, started_at: T) -> Self {
        Self {
            started_at: Some(started_at.as_ref().to_owned()),
            ..self
        }
    }

    pub fn with_ended_at<T: AsRef<str>>(self, ended_at: T) -> Self {
        Self {
            ended_at: Some(ended_at.as_ref().to_owned()),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct WatchOption {}

impl WatchOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct ListOption {}

impl ListOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct PostOption {}

impl PostOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct LogOption {}

impl LogOption {
    pub fn new() -> Self {
        Self {}
    }
}
