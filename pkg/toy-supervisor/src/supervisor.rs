use crate::msg::Request;
use crate::task::RunningTask;
use crate::{Response, RunTaskResponse, TaskResponse};
use chrono::{DateTime, Utc};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use toy_api::common::PutOption;
use toy_api::services::ServiceSpec;
use toy_api_client::client::{ServiceClient, SupervisorClient};
use toy_api_client::{ApiClient, NoopApiClient};
use toy_core::data::Frame;
use toy_core::error::ServiceError;
use toy_core::executor::{TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::mpsc::{self, Incoming, Outgoing};
use toy_core::registry::{App, Registry, ServiceSchema};
use toy_core::task::{TaskContext, TaskId};

pub fn local<TF, P>(
    factory: TF,
    app: App<P>,
) -> (
    Supervisor<TF, P, NoopApiClient>,
    Outgoing<Request, ServiceError>,
    Incoming<(), ServiceError>,
)
where
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
{
    Supervisor::new("local-supervisor", factory, app, None, None)
}

pub fn subscribe<S, TF, P, C>(
    name: S,
    factory: TF,
    app: App<P>,
    client: C,
    addr: impl Into<SocketAddr> + 'static,
) -> (
    Supervisor<TF, P, C>,
    Outgoing<Request, ServiceError>,
    Incoming<(), ServiceError>,
)
where
    S: Into<String>,
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
    C: ApiClient + Clone + Send + Sync + 'static,
{
    Supervisor::new(name, factory, app, Some(client), Some(addr.into()))
}

pub struct Supervisor<TF, P, C> {
    _factory: TF,
    app: Arc<App<P>>,
    /// receive any request.
    rx: Incoming<Request, ServiceError>,
    /// send shutdown.
    tx_shutdown: Outgoing<(), ServiceError>,
    ctx: SupervisorContext<C>,
    addr: Option<SocketAddr>,
}

#[derive(Debug, Clone)]
pub struct SupervisorContext<C> {
    name: String,
    client: Option<C>,
    tasks: Arc<Mutex<HashMap<TaskId, RunningTask>>>,
    started_at: Option<DateTime<Utc>>,
    /// send any request.
    tx: Outgoing<Request, ServiceError>,
    schemas: Vec<ServiceSchema>,
}

impl<TF, P, C> Supervisor<TF, P, C>
where
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
    C: ApiClient + Clone + Send + Sync + 'static,
{
    fn new<S: Into<String>>(
        name: S,
        factory: TF,
        app: App<P>,
        client: Option<C>,
        addr: Option<SocketAddr>,
    ) -> (
        Supervisor<TF, P, C>,
        Outgoing<Request, ServiceError>,
        Incoming<(), ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::channel::<Request, ServiceError>(1024);
        let (tx_shutdown, rx_shutdown) = mpsc::channel::<(), ServiceError>(16);
        let schemas = app.schemas();
        (
            Supervisor {
                _factory: factory,
                app: Arc::new(app),
                rx: rx_req,
                tx_shutdown,
                ctx: SupervisorContext {
                    name: name.into(),
                    client,
                    tasks: Arc::new(Mutex::new(HashMap::new())),
                    started_at: None,
                    tx: tx_req.clone(),
                    schemas,
                },
                addr,
            },
            tx_req,
            rx_shutdown,
        )
    }

    pub async fn oneshot(self, g: Graph) -> Result<RunTaskResponse, ServiceError> {
        tracing::info!(name=?self.ctx.name, "oneshot supervisor");

        let id = TaskId::new();
        let ctx = TaskContext::new(id, g);
        let (e, _) = TF::new(ctx.clone());
        e.run(&self.app, Frame::default())
            .await
            .map(|()| RunTaskResponse(id))
    }

    pub async fn run(mut self) -> Result<(), ()> {
        tracing::info!(name=?self.ctx.name, "start supervisor");

        self.ctx.started_at = Some(Utc::now());
        if let Err(_) = self.register(self.app.schemas()).await {
            return Err(());
        }

        self.spawn_server();

        // main
        while let Some(r) = self.rx.next().await {
            match r {
                Ok(m) => match m {
                    Request::RunTask(id, g, tx) => {
                        tracing::info!(name=?self.ctx.name, "receive run task request.");

                        let tasks = Arc::clone(&self.ctx.tasks);
                        let ctx = TaskContext::new(id, g);
                        let app = Arc::clone(&self.app);
                        let _ = toy_rt::spawn(async move {
                            let (e, tx_signal) = TF::new(ctx.clone());
                            let task = RunningTask::new(&ctx, tx_signal);
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.insert(ctx.id(), task);
                            }
                            let _ = e.run(&app, Frame::default()).await;
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
                            let tasks = self.ctx.tasks.lock().await;
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
                        let tasks = Arc::clone(&self.ctx.tasks);
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
                        tracing::info!(name=?self.ctx.name, "receive shutdown request.");
                        {
                            let tasks = Arc::clone(&self.ctx.tasks);
                            let mut tasks = tasks.lock().await;
                            for (_, t) in tasks.iter_mut() {
                                send_stop_signal(t).await;
                            }
                        }
                        tracing::info!(name=?self.ctx.name, "waiting all task stop....");
                        let sd = Shutdown {
                            tasks: Arc::clone(&self.ctx.tasks),
                        };
                        sd.await;
                        tracing::info!(name=?self.ctx.name, "all task stopped.");
                        break;
                    }
                },
                Err(e) => {
                    tracing::error!(name=?self.ctx.name, err = %e, "an error occured; supervisor.")
                }
            }
        }
        tracing::info!(name=?self.ctx.name, "shutdown supervisor.");
        if !self.tx_shutdown.is_closed() {
            tracing::info!(name=?self.ctx.name, "send shutdown message.");
            if let Err(e) = self.tx_shutdown.send_ok(()).await {
                tracing::error!(name=?self.ctx.name, err = %e, "an error occured; supervisor when shutdown.")
            }
        }

        Ok(())
    }

    fn spawn_server(&self) {
        if self.addr.is_none() {
            return;
        }

        let server = crate::http::Server::new(self.ctx.clone());
        let name = self.ctx.name.clone();
        let addr = self.addr.unwrap().clone();
        toy_rt::spawn(async move {
            tracing::info!(?name, "start server.");
            server.run(addr).await;
            tracing::info!(?name, "shutdown server.");
        });
    }

    async fn register(&self, schemas: Vec<ServiceSchema>) -> Result<(), ()> {
        if self.ctx.client.is_none() {
            return Ok(());
        }

        let sv = toy_api::supervisors::Supervisor::new(
            self.ctx.name.clone(),
            Utc::now(),
            Vec::new(),
            self.addr.unwrap(),
        );

        let c = self.ctx.client.as_ref().unwrap();
        if let Err(e) = c
            .supervisor()
            .put(self.ctx.name.clone(), sv, PutOption::new())
            .await
        {
            tracing::error!(name=?self.ctx.name, err = %e, "an error occured; supervisor when start up.");
            return Err(());
        }

        let specs = schemas
            .iter()
            .map(|x| {
                ServiceSpec::new(
                    x.service_type().clone(),
                    x.port_type().clone(),
                    x.schema().cloned(),
                )
            })
            .collect::<Vec<_>>();

        for spec in specs {
            let key = spec.service_type().clone();
            if let Err(e) = c
                .service()
                .put(
                    spec.service_type().full_name().to_owned(),
                    spec,
                    PutOption::new(),
                )
                .await
            {
                tracing::error!(name=?self.ctx.name, err = %e, service_type = %key, "an error occured; supervisor when register service.");
                return Err(());
            }
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

impl<TF, P, C> std::fmt::Debug for Supervisor<TF, P, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mode = if self.ctx.client.is_none() {
            "local"
        } else {
            "subscribe"
        };
        f.debug_struct("Supervisor")
            .field("name", &self.ctx.name)
            .field("mode", &mode)
            .finish()
    }
}

impl<C> SupervisorContext<C> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn tasks(&self) -> Vec<(TaskId, String)> {
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

    pub fn tx_mut(&mut self) -> &mut Outgoing<Request, ServiceError> {
        &mut self.tx
    }

    pub fn schemas(&self) -> &[ServiceSchema] {
        &self.schemas
    }
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
        std::thread::sleep(Duration::from_secs(1));
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}
