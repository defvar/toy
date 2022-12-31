use crate::task::RunningTask;
use crate::{Request, SupervisorError};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::registry::ServiceSchema;
use toy_core::task::TaskId;

#[derive(Debug, Clone)]
pub struct SupervisorContext<C> {
    name: String,
    addr: Option<SocketAddr>,
    client: Option<C>,
    tasks: Arc<Mutex<HashMap<TaskId, RunningTask>>>,
    started_at: Option<DateTime<Utc>>,
    last_event_exported_at: Arc<Mutex<Option<DateTime<Utc>>>>,
    last_task_executed_at: Arc<Mutex<Option<DateTime<Utc>>>>,
    last_metrics_exported_at: Arc<Mutex<Option<DateTime<Utc>>>>,
    /// send any request.
    tx: Outgoing<Request, ServiceError>,
    schemas: Arc<Vec<ServiceSchema>>,
    tx_http_server_shutdown: Option<Outgoing<(), SupervisorError>>,
}

impl<C> SupervisorContext<C> {
    pub fn with(
        name: impl Into<String>,
        addr: Option<SocketAddr>,
        client: Option<C>,
        tx: Outgoing<Request, ServiceError>,
        schemas: Vec<ServiceSchema>,
    ) -> SupervisorContext<C> {
        Self {
            name: name.into(),
            addr,
            client,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            started_at: None,
            last_event_exported_at: Arc::new(Mutex::new(None)),
            last_task_executed_at: Arc::new(Mutex::new(None)),
            last_metrics_exported_at: Arc::new(Mutex::new(None)),
            tx,
            schemas: Arc::new(schemas),
            tx_http_server_shutdown: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn addr(&self) -> Option<SocketAddr> {
        self.addr
    }

    pub fn client(&self) -> Option<&C> {
        self.client.as_ref()
    }

    pub fn tasks(&self) -> Arc<Mutex<HashMap<TaskId, RunningTask>>> {
        Arc::clone(&self.tasks)
    }

    pub async fn task_id_and_graph_name(&self) -> Vec<(TaskId, String)> {
        let vec = {
            let tasks = Arc::clone(&self.tasks);
            let tasks = tasks.lock().await;
            tasks
                .iter()
                .map(|x| (x.0.clone(), x.1.graph().name().to_owned()))
                .collect()
        };
        vec
    }

    pub fn started_at_str(&self) -> Option<String> {
        self.started_at.map(|x| x.to_rfc3339())
    }

    pub fn set_started_at(&mut self, v: Option<DateTime<Utc>>) {
        self.started_at = v;
    }

    pub fn tx_mut(&mut self) -> &mut Outgoing<Request, ServiceError> {
        &mut self.tx
    }

    pub fn schemas(&self) -> &[ServiceSchema] {
        &self.schemas
    }

    pub fn set_tx_http_server_shutdown(&mut self, tx: Option<Outgoing<(), SupervisorError>>) {
        self.tx_http_server_shutdown = tx;
    }

    pub fn tx_http_server_shutdown(&self) -> Option<Outgoing<(), SupervisorError>> {
        self.tx_http_server_shutdown.clone()
    }

    pub async fn last_task_executed_at(&self) -> Option<DateTime<Utc>> {
        let lock = self.last_task_executed_at.lock().await;
        lock.clone()
    }

    pub async fn task_executed(&self) {
        let mut lock = self.last_task_executed_at.lock().await;
        *lock = Some(Utc::now());
    }

    pub async fn last_event_exported_at(&self) -> Option<DateTime<Utc>> {
        let lock = self.last_event_exported_at.lock().await;
        lock.clone()
    }

    pub async fn event_exported(&self) {
        let mut lock = self.last_event_exported_at.lock().await;
        *lock = Some(Utc::now());
    }

    pub async fn last_metrics_exported_at(&self) -> Option<DateTime<Utc>> {
        let lock = self.last_metrics_exported_at.lock().await;
        lock.clone()
    }

    pub async fn metrics_exported(&self) {
        let mut lock = self.last_metrics_exported_at.lock().await;
        *lock = Some(Utc::now());
    }
}
