use crate::context::SupervisorContext;
use crate::msg::Request;
use crate::task::RunningTask;
use crate::workers::beat::beat;
use crate::workers::event_export::start_event_exporter;
use crate::workers::metrics_export::start_metrics_exporter;
use crate::{Response, RunTaskResponse, SupervisorConfig, TaskResponse};
use chrono::Utc;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use toy_api::common::{PostOption, PutOption};
use toy_api::services::ServiceSpec;
use toy_api_client::client::{ServiceClient, SupervisorClient, TaskClient};
use toy_api_client::{ApiClient, NoopApiClient};
use toy_core::data::Frame;
use toy_core::executor::{TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::metrics;
use toy_core::mpsc::{self, Incoming, Outgoing};
use toy_core::registry::{App, Registry, ServiceSchema};
use toy_core::task::{TaskContext, TaskId};

pub fn local<TF, P, Config>(
    factory: TF,
    app: App<P>,
    config: Config,
) -> (
    Supervisor<TF, P, NoopApiClient, Config>,
    Outgoing<Request>,
    Incoming<()>,
)
where
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
    Config: SupervisorConfig + Clone + Send + Sync + 'static,
{
    Supervisor::new("local-supervisor", factory, app, None, None, config)
}

pub fn subscribe<S, TF, P, C, Config>(
    name: S,
    factory: TF,
    app: App<P>,
    client: C,
    addr: impl Into<SocketAddr> + 'static,
    config: Config,
) -> (
    Supervisor<TF, P, C, Config>,
    Outgoing<Request>,
    Incoming<()>,
)
where
    S: Into<String>,
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
    C: ApiClient + Clone + Send + Sync + 'static,
    Config: SupervisorConfig + Clone + Send + Sync + 'static,
{
    Supervisor::new(name, factory, app, Some(client), Some(addr.into()), config)
}

pub struct Supervisor<TF, P, C, Config> {
    _factory: TF,
    app: Arc<App<P>>,
    /// receive any request.
    rx: Incoming<Request>,
    /// send shutdown.
    tx_shutdown: Outgoing<()>,
    ctx: SupervisorContext<C>,
    config: Config,
}

impl<TF, P, C, Config> Supervisor<TF, P, C, Config>
where
    TF: TaskExecutorFactory + Send + 'static,
    P: Registry + 'static,
    C: ApiClient + Clone + Send + Sync + 'static,
    Config: SupervisorConfig + Clone + Send + Sync + 'static,
{
    fn new<S: Into<String>>(
        name: S,
        factory: TF,
        app: App<P>,
        client: Option<C>,
        addr: Option<SocketAddr>,
        config: Config,
    ) -> (
        Supervisor<TF, P, C, Config>,
        Outgoing<Request>,
        Incoming<()>,
    ) {
        let (tx_req, rx_req) = mpsc::channel::<Request>(1024);
        let (tx_shutdown, rx_shutdown) = mpsc::channel::<()>(16);
        let schemas = app.schemas();
        (
            Supervisor {
                _factory: factory,
                app: Arc::new(app),
                rx: rx_req,
                tx_shutdown,
                ctx: SupervisorContext::with(name, addr, client, tx_req.clone(), schemas),
                config,
            },
            tx_req,
            rx_shutdown,
        )
    }

    pub async fn oneshot(
        self,
        g: Graph,
    ) -> Result<RunTaskResponse, <<TF as TaskExecutorFactory>::Executor as TaskExecutor>::Error>
    {
        tracing::info!(name=?self.ctx.name(), "oneshot supervisor");

        let id = TaskId::new();
        let ctx = TaskContext::new(id, g);
        let (e, _) = TF::new(ctx.clone());
        e.run(&self.app, Frame::default())
            .await
            .map(|()| RunTaskResponse(id))
    }

    pub async fn run(mut self) -> Result<(), ()> {
        tracing::info!(name=?self.ctx.name(), "start supervisor");

        self.ctx.set_started_at(Some(Utc::now()));
        if let Err(_) = self.register(self.app.schemas()).await {
            return Err(());
        }

        self.spawn_workers();

        // main
        while let Some(r) = self.rx.next().await {
            match r {
                Request::RunTask(id, g, tx) => {
                    tracing::info!(name=?self.ctx.name(), "receive run task request.");

                    let app = Arc::clone(&self.app);
                    let ctx = self.ctx.clone();
                    let _ = toy_rt::spawn_named(
                        async move {
                            let tasks = ctx.tasks();
                            let events = metrics::context::events_by_task_id(id).await;
                            let task_ctx = TaskContext::with_parts(id, g, events);

                            let (executor, tx_signal) = TF::new(task_ctx.clone());
                            let task = RunningTask::new(&task_ctx, tx_signal);
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.insert(task_ctx.id(), task);
                            }
                            let task_execution_error = executor.run(&app, Frame::default()).await;
                            if let Err(e) = task_execution_error {
                                tracing::error!(name=?task_ctx.name(), err = %e, "an error occured while task execution.")
                            }
                            if let Err(e) = ctx
                                .client()
                                .unwrap()
                                .task()
                                .finish(task_ctx.id(), PostOption::new())
                                .await
                            {
                                tracing::error!(name=?task_ctx.name(), err = %e, "an error occurred while change the status of a task to \"Finish\".")
                            }
                            {
                                let mut tasks = tasks.lock().await;
                                let _ = tasks.remove(&task_ctx.id());
                            }
                            ctx.task_executed().await;
                            ()
                        },
                        "supervisor-runTask",
                    );
                    let _ = tx.send(RunTaskResponse(id)).await;
                }
                Request::Tasks(tx) => {
                    let r = {
                        let tasks = self.ctx.tasks();
                        let tasks = tasks.lock().await;
                        tasks
                            .iter()
                            .map(|(_, v)| TaskResponse::from(v))
                            .collect::<Vec<_>>()
                    };
                    let _ = tx.send(r).await;
                }
                Request::Task(id, tx) => {
                    let r = {
                        let tasks = self.ctx.tasks();
                        let tasks = tasks.lock().await;
                        tasks
                            .iter()
                            .filter(|(i, _)| id == *i)
                            .nth(0)
                            .map(|(_, v)| TaskResponse::from(v))
                    };
                    let _ = tx.send(r).await;
                }
                Request::Stop(uuid) => {
                    let tasks = self.ctx.tasks();
                    let mut tasks = tasks.lock().await;
                    if let Some(t) = tasks.get_mut(&uuid) {
                        send_stop_signal(t).await;
                    }
                }
                Request::Services(tx) => {
                    let m = Response::Services(self.app.schemas());
                    let _ = tx.send(m).await;
                }
                Request::Shutdown => {
                    tracing::info!(name=?self.ctx.name(), "receive shutdown request.");

                    if let Some(mut tx) = self.ctx.tx_http_server_shutdown() {
                        tracing::info!(name=?self.ctx.name(), "shutdown api server...");
                        let _ = tx.send_ok(()).await;
                    }

                    tracing::info!(name=?self.ctx.name(), "send stop signal all tasks.");
                    {
                        let tasks = self.ctx.tasks();
                        let mut tasks = tasks.lock().await;
                        for (_, t) in tasks.iter_mut() {
                            send_stop_signal(t).await;
                        }
                    }
                    tracing::info!(name=?self.ctx.name(), "waiting all task stop....");
                    let sd = Shutdown {
                        tasks: Arc::clone(&self.ctx.tasks()),
                    };
                    sd.await;
                    tracing::info!(name=?self.ctx.name(), "all task stopped.");
                    break;
                }
            }
        }
        tracing::info!(name=?self.ctx.name(), "shutdown supervisor.");
        if !self.tx_shutdown.is_closed() {
            tracing::info!(name=?self.ctx.name(), "send shutdown message.");
            if let Err(e) = self.tx_shutdown.send_ok(()).await {
                tracing::error!(name=?self.ctx.name(), err = %e, "an error occured; supervisor when shutdown.")
            }
        }

        Ok(())
    }

    fn spawn_workers(&mut self) {
        if self.ctx.addr().is_none() {
            return;
        }

        let ctx = self.ctx.clone();
        let addr = ctx.addr().unwrap().clone();
        let name = ctx.name().to_string();
        let client = ctx.client().clone().unwrap();
        let config = self.config.clone();
        let (tx, rx) = mpsc::channel::<()>(10);
        self.ctx.set_tx_http_server_shutdown(Some(tx));
        let beat_interval = config.heart_beat_interval_mills();
        let event_interval = config.event_export_interval_mills();
        let metrics_interval = config.metrics_export_interval_mills();
        let event_exporter = config.event_exporter();
        let metrics_exporer = config.metrics_exporter();

        {
            let ctx = ctx.clone();
            let n = name.clone();
            toy_rt::spawn_named(
                async move {
                    tracing::info!(name=?n, "start server.");
                    let server = crate::http::Server::new(ctx);
                    server.run(addr, config, rx).await;
                    tracing::info!(name=?n, "stop server.");
                },
                "supervisor-api-serve",
            );
        }
        {
            let c = client.clone();
            let n = name.clone();
            toy_rt::spawn_named(
                async move {
                    tracing::info!(name=?n, "start beater.");
                    beat(c, &n, beat_interval).await;
                    tracing::info!(name=?n, "stop beater.");
                },
                "supervisor-beat",
            );
        }
        {
            let ctx = ctx.clone();
            let n = name.clone();
            toy_rt::spawn_named(
                async move {
                    tracing::info!(name=?n, "start event exporter.");
                    start_event_exporter(ctx, event_exporter, event_interval).await;
                    tracing::info!(name=?n, "stop event exporter.");
                },
                "supervisor-event-exporter",
            );
        }
        {
            let ctx = ctx.clone();
            let n = name.clone();
            toy_rt::spawn_named(
                async move {
                    tracing::info!(name=?n, "start metrics exporter.");
                    start_metrics_exporter(ctx, metrics_exporer, metrics_interval).await;
                    tracing::info!(name=?n, "stop metrics exporter.");
                },
                "supervisor-metrics-exporter",
            );
        }
    }

    async fn register(&self, schemas: Vec<ServiceSchema>) -> Result<(), ()> {
        if self.ctx.client().is_none() {
            return Ok(());
        }

        let sv = toy_api::supervisors::Supervisor::new(
            self.ctx.name().to_string(),
            Utc::now(),
            Vec::new(),
            self.ctx.addr().unwrap(),
        );

        let c = self.ctx.client().unwrap();
        if let Err(e) = c
            .supervisor()
            .put(self.ctx.name().to_string(), sv, PutOption::new())
            .await
        {
            tracing::error!(name=?self.ctx.name(), err = %e, "an error occured; supervisor when start up.");
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
                tracing::error!(name=?self.ctx.name(), err = %e, service_type = %key, "an error occured; supervisor when register service.");
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

impl<TF, P, C, Config> std::fmt::Debug for Supervisor<TF, P, C, Config> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mode = if self.ctx.client().is_none() {
            "local"
        } else {
            "subscribe"
        };
        f.debug_struct("Supervisor")
            .field("name", &self.ctx.name())
            .field("mode", &mode)
            .finish()
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
