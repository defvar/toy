use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::{AsyncSpawner, Executor};
use crate::graph::Graph;
use crate::mpsc::{self, Incoming, Outgoing, OutgoingMessage};
use crate::oneshot;
use crate::registry::{App, Delegator, Registry, ServiceSchema};
use crate::task::{RunningTask, TaskContext};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub enum Request {
    Task(Graph, oneshot::Outgoing<TaskResponse, ServiceError>),
    Stop(Uuid),
    Services(oneshot::Outgoing<Response, ServiceError>),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    Services(Vec<ServiceSchema>),
}

#[derive(Debug, Clone)]
pub struct TaskResponse(Uuid);

impl TaskResponse {
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemMessage {
    Shutdown,
}

impl OutgoingMessage for Request {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for Response {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for SystemMessage {
    fn set_port(&mut self, _port: u8) {}
}

#[derive(Debug)]
pub struct Supervisor<T, O, P> {
    spawner: T,

    app: App<O, P>,

    /// send system message to api server.
    tx: Outgoing<SystemMessage, ServiceError>,

    /// receive any request from api server.
    rx: Incoming<Request, ServiceError>,

    tasks: Arc<Mutex<HashMap<Uuid, RunningTask>>>,
}

impl<T, O, P> Supervisor<T, O, P>
where
    T: AsyncSpawner + 'static,
    O: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Registry
        + Clone
        + Send
        + 'static,
    P: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Registry
        + Clone
        + Send
        + 'static,
{
    pub fn new(
        spawner: T,
        registry: App<O, P>,
    ) -> (
        Supervisor<T, O, P>,
        Outgoing<Request, ServiceError>,
        Incoming<SystemMessage, ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::channel::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::channel::<SystemMessage, ServiceError>(1024);
        (
            Supervisor {
                spawner,
                app: registry,
                tx: tx_sys,
                rx: rx_req,
                tasks: Arc::new(Mutex::new(HashMap::new())),
            },
            tx_req,
            rx_sys,
        )
    }

    pub async fn run(mut self) -> Result<(), ()> {
        tracing::info!("start supervisor");

        // main
        while let Some(r) = self.rx.next().await {
            match r {
                Ok(m) => match m {
                    Request::Task(g, tx) => {
                        let app = self.app.clone();
                        let tasks = Arc::clone(&self.tasks);
                        let s = self.spawner.clone();
                        let ctx = TaskContext::new(g);
                        let uuid = ctx.uuid();
                        let _ = self.spawner.spawn(async move {
                            let (e, tx_signal) = Executor::new(s, ctx.clone());
                            tracing::info!(uuid = ?ctx.uuid(), "start task.");
                            let task = RunningTask::new(&ctx, tx_signal);
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.insert(ctx.uuid(), task);
                                tracing::debug!(uuid = ?ctx.uuid(), "add task store.");
                            }
                            let _ = e.run(app, Frame::default()).await;
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.remove(&ctx.uuid());
                                tracing::debug!(uuid = ?ctx.uuid(), "remove task store.");
                            }
                            tracing::info!(uuid = ?ctx.uuid(), "end task.");
                            ()
                        });
                        let _ = tx.send_ok(TaskResponse(uuid)).await;
                    }
                    Request::Stop(uuid) => {
                        let tasks = Arc::clone(&self.tasks);
                        let mut tasks = tasks.lock().await;
                        if let Some(t) = tasks.get_mut(&uuid) {
                            send_stop_signal(t).await;
                        }
                    }
                    Request::Services(tx) => {
                        let m = Response::Services(self.app.schemas());
                        let _ = tx.send_ok(m).await;
                    }
                    Request::Shutdown => {
                        tracing::info!("receive shutdown request.");
                        {
                            let tasks = Arc::clone(&self.tasks);
                            let mut tasks = tasks.lock().await;
                            for (_, t) in tasks.iter_mut() {
                                send_stop_signal(t).await;
                            }
                        }
                        tracing::info!("waiting all task stop....");
                        let sd = Shutdown {
                            tasks: Arc::clone(&self.tasks),
                        };
                        sd.await;
                        tracing::info!("all task stopped.");
                        break;
                    }
                },
                Err(e) => tracing::error!(err = ?e, "an error occured; supervisor."),
            }
        }
        tracing::info!("shutdown supervisor");
        let _ = self.tx.send_ok(SystemMessage::Shutdown).await;
        Ok(())
    }
}

async fn send_stop_signal(task: &mut RunningTask) {
    for (uri, tx) in task.tx_signal().iter_mut() {
        for p in tx.ports() {
            let r = tx.send_ok_to(p, Frame::stop()).await;
            tracing::debug!(
                uri = ?uri,
                port = p,
                ret = ?r,
                "send stop signal.",
            );
        }
    }
    tracing::info!(uuid = ?task.uuid(), "send stop signal to task.");
}

struct Shutdown {
    tasks: Arc<Mutex<HashMap<Uuid, RunningTask>>>,
}

impl Unpin for Shutdown {}

impl Future for Shutdown {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        {
            match self.tasks.try_lock() {
                Ok(tasks) => {
                    if tasks.is_empty() {
                        return Poll::Ready(());
                    }
                }
                _ => (),
            }
        }
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}
