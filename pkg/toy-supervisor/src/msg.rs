use std::time::Duration;
use toy_core::mpsc::OutgoingMessage;
use toy_core::oneshot;
use toy_core::prelude::{Graph, ServiceError, TaskId};
use toy_core::registry::ServiceSchema;

#[derive(Debug)]
pub enum Request {
    RunTask(Graph, oneshot::Outgoing<RunTaskResponse, ServiceError>),
    Tasks(oneshot::Outgoing<Vec<TaskResponse>, ServiceError>),
    Stop(TaskId),
    Services(oneshot::Outgoing<Response, ServiceError>),
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

#[derive(Debug, Clone)]
pub struct TaskResponse {
    pub(crate) id: TaskId,
    pub(crate) started_at: Duration,
    pub(crate) graph: Graph,
}

impl TaskResponse {
    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn started_at(&self) -> Duration {
        self.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}

impl OutgoingMessage for Request {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for Response {
    fn set_port(&mut self, _port: u8) {}
}
