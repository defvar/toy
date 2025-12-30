use std::time::SystemTime;
use toy_core::graph::Graph;
use toy_core::node_channel::SignalOutgoings;
use toy_core::task::{TaskContext, TaskId};

/// Infomation Of Running Task.
/// Use Actor.
#[derive(Debug)]
pub struct RunningTask {
    id: TaskId,
    started_at: SystemTime,
    graph: Graph,

    /// use running task.
    tx_signal: SignalOutgoings,
}

impl RunningTask {
    pub fn new(ctx: &TaskContext, tx_signal: SignalOutgoings) -> Self {
        Self {
            id: ctx.id(),
            started_at: ctx.started_at(),
            graph: ctx.graph().clone(),
            tx_signal,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn started_at(&self) -> SystemTime {
        self.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn tx_signal(&mut self) -> &mut SignalOutgoings {
        &mut self.tx_signal
    }
}
