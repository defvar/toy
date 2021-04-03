use crate::msg::Request;
use crate::task::RunningTask;
use crate::{Response, RunTaskResponse, TaskResponse};
use chrono::{DateTime, Utc};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use toy_api::service::ServicesEntity;
use toy_api::supervisors::PutOption;
use toy_api_client::client::SupervisorClient;
use toy_api_client::{ApiClient, NoopApiClient};
use toy_core::data::Frame;
use toy_core::error::ServiceError;
use toy_core::executor::{TaskExecutor, TaskExecutorFactory};
use toy_core::mpsc::{self, Incoming, Outgoing};
use toy_core::registry::{App, Delegator, Registry};
use toy_core::task::{TaskContext, TaskId};

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
    Supervisor::new("single-supervisor", factory, app, None)
}

pub fn spawn<S, TF, O, P, C>(
    name: S,
    factory: TF,
    app: App<O, P>,
    client: C,
) -> (
    Supervisor<TF, O, P, C>,
    Outgoing<Request, ServiceError>,
    Incoming<(), ServiceError>,
)
where
    S: Into<String>,
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
    C: ApiClient + Clone + Send + Sync + 'static,
{
    Supervisor::new(name, factory, app, Some(client))
}

#[derive(Debug)]
pub struct Supervisor<TF, O, P, C> {
    name: String,

    factory: TF,

    app: App<O, P>,

    client: Option<C>,

    /// receive any request.
    rx: Incoming<Request, ServiceError>,

    /// send shutdown.
    tx: Outgoing<(), ServiceError>,

    /// use watcher.
    tx_watcher: Outgoing<Request, ServiceError>,

    tasks: Arc<Mutex<HashMap<TaskId, RunningTask>>>,

    started_at: Option<DateTime<Utc>>,
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
    C: ApiClient + Clone + Send + Sync + 'static,
{
    fn new<S: Into<String>>(
        name: S,
        factory: TF,
        app: App<O, P>,
        client: Option<C>,
    ) -> (
        Supervisor<TF, O, P, C>,
        Outgoing<Request, ServiceError>,
        Incoming<(), ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::channel::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::channel::<(), ServiceError>(16);
        (
            Supervisor {
                name: name.into(),
                factory,
                app,
                client,
                rx: rx_req,
                tx: tx_sys,
                tx_watcher: tx_req.clone(),
                tasks: Arc::new(Mutex::new(HashMap::new())),
                started_at: None,
            },
            tx_req,
            rx_sys,
        )
    }

    pub async fn run(mut self) -> Result<(), ()> {
        tracing::info!("start supervisor");

        self.started_at = Some(Utc::now());
        if let Err(_) = self.register().await {
            return Err(());
        }

        self.spawn_watcher();

        // main
        while let Some(r) = self.rx.next().await {
            match r {
                Ok(m) => match m {
                    Request::RunTask(id, g, tx) => {
                        let app = self.app.clone();
                        let tasks = Arc::clone(&self.tasks);
                        let ctx = TaskContext::new(id, g);
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
                        let _ = tx.send_ok(RunTaskResponse(id)).await;
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

    fn spawn_watcher(&self) {
        if self.client.is_none() {
            return;
        }

        let c = self.client.as_ref().map(|x| x.clone()).unwrap();
        let tx = self.tx_watcher.clone();
        let name = self.name.clone();

        toy_rt::spawn(async move {
            tracing::info!("start watch task.");
            if let Err(e) = super::watcher::watch(name, c, tx).await {
                tracing::error!(err = ?e, "an error occured; supervisor when watch task.");
            }
            tracing::info!("shutdown watcher.");
        });
    }

    async fn register(&self) -> Result<(), ()> {
        if self.client.is_none() {
            return Ok(());
        }

        let services = ServicesEntity::new(self.app.schemas());
        let start_time = self.started_at.unwrap().to_rfc3339();
        let sv = toy_api::supervisors::Supervisor::new(
            self.name.clone(),
            start_time,
            Vec::new(),
            services,
        );

        let c = self.client.as_ref().unwrap();
        if let Err(e) = c
            .supervisor()
            .put(self.name.clone(), sv, PutOption::default())
            .await
        {
            tracing::error!(err = ?e, "an error occured; supervisor when start up.");
            return Err(());
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
