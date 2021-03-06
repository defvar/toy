use crate::common::Format;
use crate::graph::GraphEntity;
use toy_core::prelude::TaskId;
use toy_pack::{Pack, Unpack};

#[derive(Debug, Clone, Pack, Unpack)]
pub enum PendingStatus {
    Created,
    Allocated,
}

#[derive(Debug, Clone, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct PendingEntity {
    task_id: TaskId,
    status: PendingStatus,
    allocated_supervisor: Option<String>,
    allocated_at: Option<String>,
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

#[derive(Debug, Clone, Pack, Unpack)]
pub struct AllocateRequest {
    name: String,
}

#[derive(Debug, Clone, Pack, Unpack)]
pub struct AllocateResponse {
    task_id: TaskId,
    status: AllocateStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, Unpack)]
pub enum AllocateStatus {
    Ok,
    NotFound,
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

impl Default for PendingStatus {
    fn default() -> Self {
        PendingStatus::Created
    }
}

impl PendingEntity {
    pub fn new(task_id: TaskId, graph: GraphEntity) -> Self {
        Self {
            task_id,
            status: PendingStatus::Created,
            allocated_supervisor: None,
            allocated_at: None,
            graph: Some(graph),
        }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn status(&self) -> &PendingStatus {
        &self.status
    }

    pub fn graph(&self) -> Option<&GraphEntity> {
        self.graph.as_ref()
    }

    pub fn allocate<S: Into<String>, T: Into<String>>(self, name: S, allocated_at: T) -> Self {
        Self {
            task_id: self.task_id,
            graph: self.graph,
            status: PendingStatus::Allocated,
            allocated_supervisor: Some(name.into()),
            allocated_at: Some(allocated_at.into()),
        }
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

impl AllocateRequest {
    pub fn new<P: Into<String>>(name: P) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AllocateResponse {
    pub fn ok(id: TaskId) -> Self {
        Self {
            task_id: id,
            status: AllocateStatus::Ok,
        }
    }

    pub fn not_found(id: TaskId) -> Self {
        Self {
            task_id: id,
            status: AllocateStatus::NotFound,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == AllocateStatus::Ok
    }
}

impl Default for AllocateStatus {
    fn default() -> Self {
        AllocateStatus::NotFound
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

/// Watch api option.
#[derive(Clone, Debug, Pack, Unpack)]
pub struct WatchOption {
    format: Option<Format>,
}

impl WatchOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// List api option.
#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Post api option.
#[derive(Clone, Debug, Pack, Unpack)]
pub struct PostOption {
    format: Option<Format>,
}

impl PostOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Allocate api option.
#[derive(Clone, Debug, Pack, Unpack)]
pub struct AllocateOption {
    format: Option<Format>,
}

impl AllocateOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Log api option.
#[derive(Clone, Debug, Pack, Unpack)]
pub struct LogOption {
    format: Option<Format>,
}

impl LogOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}
