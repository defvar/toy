use crate::task::RunningTask;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use toy_api_client::{ApiClient, NoopApiClient};
use toy_core::data::Frame;
use toy_core::error::ServiceError;
use toy_core::executor::{TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::mpsc::{self, Incoming, Outgoing, OutgoingMessage};
use toy_core::oneshot;
use toy_core::registry::{App, Delegator, Registry, ServiceSchema};
use toy_core::task::{TaskContext, TaskId};

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
pub struct RunTaskResponse(TaskId);

impl RunTaskResponse {
    pub fn id(&self) -> TaskId {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct TaskResponse {
    id: TaskId,
    started_at: Duration,
    graph: Graph,
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

pub fn single<TF, O, P>(
    factory: TF,
    app: App<O, P>,
) -> (
    Supervisor<TF, O, P, NoopApiClient>,
    Outgoing<Request, ServiceError>,
    Incoming<(), ServiceError>,
)
where
    TF: TaskExecutorFactory + Send,
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
    Supervisor::new(factory, app, NoopApiClient)
}

#[derive(Debug)]
pub struct Supervisor<TF, O, P, C> {
    factory: TF,

    app: App<O, P>,

    client: Option<C>,

    /// receive any request.
    rx: Incoming<Request, ServiceError>,

    /// send shutdown.
    tx: Outgoing<(), ServiceError>,

    tasks: Arc<Mutex<HashMap<TaskId, RunningTask>>>,
}

impl<TF, O, P, C> Supervisor<TF, O, P, C>
where
    TF: TaskExecutorFactory + Send,
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
    C: ApiClient,
{
    pub fn new(
        factory: TF,
        app: App<O, P>,
        client: C,
    ) -> (
        Supervisor<TF, O, P, C>,
        Outgoing<Request, ServiceError>,
        Incoming<(), ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::channel::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::channel::<(), ServiceError>(16);
        (
            Supervisor {
                factory,
                app,
                client: Some(client),
                rx: rx_req,
                tx: tx_sys,
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
                    Request::RunTask(g, tx) => {
                        let app = self.app.clone();
                        let tasks = Arc::clone(&self.tasks);
                        let ctx = TaskContext::new(g);
                        let uuid = ctx.id();
                        let _ = toy_rt::spawn(async move {
                            let (e, tx_signal) = TF::new(ctx.clone());
                            let task = RunningTask::new(&ctx, tx_signal);
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.insert(ctx.id(), task);
                            }
                            let _ = e.run(app, Frame::default()).await;
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.remove(&ctx.id());
                            }
                            ()
                        });
                        let _ = tx.send_ok(RunTaskResponse(uuid)).await;
                    }
                    Request::Tasks(tx) => {
                        let r = {
                            let tasks = self.tasks.lock().await;
                            tasks
                                .iter()
                                .map(|(_, v)| TaskResponse {
                                    id: v.id(),
                                    started_at: v.started_at(),
                                    graph: v.graph().clone(),
                                })
                                .collect::<Vec<_>>()
                        };
                        let _ = tx.send_ok(r).await;
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
        if let Err(e) = self.tx.send_ok(()).await {
            tracing::error!(err = ?e, "an error occured; supervisor when shutdown.")
        }

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
    tracing::info!(uuid = ?task.id(), "send stop signal to task.");
}

struct Shutdown {
    tasks: Arc<Mutex<HashMap<TaskId, RunningTask>>>,
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
