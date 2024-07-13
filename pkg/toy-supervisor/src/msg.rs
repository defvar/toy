use std::time::SystemTime;
use serde::Serialize;
use toy_core::data::Value;
use toy_core::mpsc::OutgoingMessage;
use toy_core::oneshot;
use toy_core::prelude::{Graph, TaskId};
use toy_core::registry::ServiceSchema;
use crate::task::RunningTask;

#[derive(Debug)]
pub enum Request {
    RunTask(TaskId, Graph, oneshot::Outgoing<RunTaskResponse>),
    Tasks(oneshot::Outgoing<Vec<TaskResponse>>),
    Task(TaskId, oneshot::Outgoing<Option<TaskResponse>>),
    Stop(TaskId),
    Services(oneshot::Outgoing<Response>),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    Services(Vec<ServiceSchema>),
}

#[derive(Debug, Clone)]
pub struct RunTaskResponse(pub(crate) TaskId);

impl RunTaskResponse {
    pub fn id(&self) -> TaskId {
        self.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskResponse {
    id: TaskId,
    started_at: SystemTime,
    graph: Value,
    config: Value,
}

impl TaskResponse {
    pub fn from(v: &RunningTask) -> TaskResponse {
        Self {
            id: v.id(),
            started_at: v.started_at(),
            graph: v.graph().original(),
            config: v.graph().config(),
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn started_at(&self) -> SystemTime {
        self.started_at
    }

    pub fn graph(&self) -> &Value {
        &self.graph
    }

    pub fn config(&self) -> &Value {
        &self.config
    }
}

impl OutgoingMessage for Request {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for Response {
    fn set_port(&mut self, _port: u8) {}
}
