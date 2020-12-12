use crate::graph::Graph;
use crate::node_channel::SignalOutgoings;
use crate::Uri;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// You do **not** have to wrap the `TaskContext` it in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Clone)]
pub struct TaskContext {
    inner: Arc<Inner>,
    uri: Option<Uri>,
}

struct Inner {
    uuid: Uuid,
    started_at: Duration,
    graph: Graph,
}

#[derive(Debug)]
pub struct RunningTask {
    uuid: Uuid,
    started_at: Duration,
    graph: Graph,

    /// use running task.
    tx_signal: SignalOutgoings,
}

impl TaskContext {
    pub fn new(graph: Graph) -> Self {
        Self {
            inner: Arc::new(Inner {
                uuid: Uuid::new_v4(),
                started_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards"),
                graph,
            }),
            uri: None,
        }
    }

    pub fn with_uri(self, uri: &Uri) -> Self {
        Self {
            inner: self.inner,
            uri: Some(uri.clone()),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.inner.uuid
    }

    pub fn started_at(&self) -> Duration {
        self.inner.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.inner.graph
    }

    pub fn uri(&self) -> Option<Uri> {
        self.uri.as_ref().map(|x| x.clone())
    }

    pub fn trace_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::TRACE, "Task", task=?self.uuid(), uri=?uri),
            None => tracing::span!(tracing::Level::TRACE, "Task", task=?self.uuid()),
        }
    }

    pub fn debug_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::DEBUG, "Task", task=?self.uuid(), uri=?uri),
            None => tracing::span!(tracing::Level::DEBUG, "Task", task=?self.uuid()),
        }
    }

    pub fn info_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::INFO, "Task", task=?self.uuid(), uri=?uri),
            None => tracing::span!(tracing::Level::INFO, "Task", task=?self.uuid()),
        }
    }
}

impl fmt::Debug for TaskContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TaskContext")
            .field("uuid", &self.inner.uuid)
            .field("started_at", &self.inner.started_at)
            .field("graph", &self.inner.graph)
            .finish()
    }
}

impl RunningTask {
    pub fn new(ctx: &TaskContext, tx_signal: SignalOutgoings) -> Self {
        Self {
            uuid: ctx.uuid(),
            started_at: ctx.started_at(),
            graph: ctx.graph().clone(),
            tx_signal,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn started_at(&self) -> Duration {
        self.started_at
    }

    pub fn tx_signal(&mut self) -> &mut SignalOutgoings {
        &mut self.tx_signal
    }
}
