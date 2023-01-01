use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use toy_core::data::{self, Frame};
use toy_core::error::{Error, OutgoingError, ServiceError};
use toy_core::executor::{ServiceExecutor, TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::metrics::MetricsEventKind;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::node_channel::{self, Awaiter, Incomings, Outgoings, SignalOutgoings, Starters};
use toy_core::registry::{App, ExecuteResult, Registry};
use toy_core::service::{Service, ServiceContext, ServiceFactory};
use toy_core::task::TaskContext;
use toy_core::ServiceType;
use toy_core::Uri;
use tracing::Span;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    StartService,
    FinishService,
    StartTask,
    FinishTask,
}

impl Operation {
    pub fn as_message(&self) -> &str {
        match self {
            Operation::StartService => "start service.",
            Operation::FinishService => "finish service.",
            Operation::StartTask => "start task.",
            Operation::FinishTask => "finish task.",
        }
    }
}

impl Into<String> for Operation {
    fn into(self) -> String {
        self.as_message().to_string()
    }
}

/// Common implementation `TaskExecutor` and `ServiceExecutor`.
pub struct Executor {
    starters: Starters,
    awaiter: Awaiter,
    graph: Graph,
    incomings: Incomings,
    outgoings: Outgoings,
    ctx: TaskContext,
    errors: Arc<Mutex<Vec<ServiceError>>>,
}

/// Common implementation `TaskExecutorFactory`.
pub struct ExecutorFactory;

impl Executor {
    pub fn new(ctx: TaskContext) -> (Self, SignalOutgoings) {
        let graph = ctx.graph().clone();
        let (incomings, outgoings, awaiter, starters, signals) = node_channel::from_graph(&graph);
        (
            Self {
                starters,
                awaiter,
                graph,
                incomings,
                outgoings,
                ctx,
                errors: Arc::new(Mutex::new(Vec::new())),
            },
            signals,
        )
    }

    fn pop_channels(&mut self, uri: &Uri) -> (Outgoing<Frame>, (Incoming<Frame>, u32)) {
        if let Some(tx) = self.outgoings.pop(uri) {
            if let Some(rx) = self.incomings.pop(uri) {
                return (tx, rx);
            }
        }
        panic!("invalid channel stack...");
    }

    async fn run_0(mut self, start_frame: Frame) -> Result<(), OutgoingError> {
        for (_, tx) in &mut self.starters.iter_mut() {
            tx.send(start_frame.clone()).await?
        }

        let uri: Uri = "awaiter".into();
        let awaiter_ctx = self.ctx.with_uri(&uri);
        let span = awaiter_ctx.info_span();
        let mut finish_count = 0;
        while let Some(req) = self.awaiter.next().await {
            if req.is_stop() {
                tracing::info!(parent: &span, ?uri, "receive stop signal.");
                break;
            }
            if req.is_upstream_finish() {
                finish_count += 1;
                tracing::info!(
                    parent: &span,
                    ?uri,
                    finish_count,
                    upstream_count= ?self.awaiter.upstream_count(),
                    "receive upstream finish signal."
                );
                if finish_count >= self.awaiter.upstream_count() {
                    tracing::info!(parent: &span, ?uri, "all upstream finish.");
                    break;
                }
            }
        }
        awaiter_ctx
            .push_task_event(MetricsEventKind::FinishTask)
            .await;
        Ok(())
    }
}

impl ServiceExecutor for Executor {
    type Request = Frame;

    fn spawn<F>(&mut self, service_type: &ServiceType, uri: &Uri, factory: F)
    where
        F: ServiceFactory<Request = Self::Request> + Send + Sync + 'static,
        F::Service: Send,
        F::Context: Send,
        F::Config: DeserializeOwned + Send,
    {
        let (tx, (rx, upstream_count)) = self.pop_channels(uri);
        let uri = uri.clone();
        let service_type = service_type.clone();

        let config_value = self.graph.by_uri(&uri);
        if config_value.is_none() {
            tracing::error!(?uri, "not found service.");
            return;
        }

        let task_ctx = self.ctx.clone();
        let task_ctx = task_ctx.with_uri(&uri);
        let errors = Arc::clone(&self.errors);
        match data::unpack::<F::Config>(&config_value.unwrap().config()) {
            Ok(config) => {
                let task_name = uri.to_string();
                toy_rt::spawn_named(
                    async move {
                        let new_service = factory.new_service(service_type.clone()).await;
                        let new_ctx = factory.new_context(service_type.clone(), config).await;
                        match (new_service, new_ctx) {
                            (Ok(service), Ok(ctx)) => {
                                if let Err(e) = process(
                                    task_ctx,
                                    rx,
                                    upstream_count,
                                    tx,
                                    service,
                                    service_type.clone(),
                                    ctx,
                                )
                                .await
                                {
                                    push_error(
                                        Arc::clone(&errors),
                                        &uri,
                                        &service_type,
                                        ServiceError::error(e),
                                    )
                                    .await;
                                }
                            }
                            (s, c) => {
                                if let Some(e) = s.err() {
                                    push_error(
                                        Arc::clone(&errors),
                                        &uri,
                                        &service_type,
                                        ServiceError::service_init_failed(&uri, &service_type, e),
                                    )
                                    .await;
                                }
                                if let Some(e) = c.err() {
                                    push_error(
                                        Arc::clone(&errors),
                                        &uri,
                                        &service_type,
                                        ServiceError::context_init_failed(&uri, &service_type, e),
                                    )
                                    .await;
                                }
                            }
                        }
                    },
                    &format!("task-{}", task_name),
                );
            }
            Err(e) => tracing::error!("config initialize failed. uri:{:?}, error:{:?}", uri, e),
        }
    }
}

#[async_trait]
impl TaskExecutor for Executor {
    type Error = ServiceError;

    async fn run<T>(mut self, app: &App<T>, start_frame: Frame) -> Result<(), Self::Error>
    where
        T: Registry,
    {
        let span = self.ctx.info_span();
        let started_at = self.ctx.started_at();

        tracing::info!(
            parent: &span,
            operation=?Operation::StartTask,
            "{}",
            Operation::StartTask.as_message()
        );
        self.ctx.push_task_event(MetricsEventKind::StartTask).await;
        // need to reverse ....
        let nodes = self
            .graph
            .iter()
            .rev()
            .map(|x| (x.service_type(), x.uri()))
            .collect::<Vec<_>>();

        for (tp, uri) in &nodes {
            if app.delegate(tp, uri, &mut self) == ExecuteResult::NotFound {
                tracing::error!(parent: &span, ?uri, "service not found.");
                return Err(Error::service_not_found(tp.clone()));
            }
        }

        let errors = Arc::clone(&self.errors);
        let r = self.run_0(start_frame).await;
        if let Err(e) = r {
            let mut lock = errors.lock().await;
            lock.push(e.into());
        }

        log_total_time(started_at, &span, None, Operation::FinishTask);

        {
            let lock = errors.lock().await;
            if lock.is_empty() {
                Ok(())
            } else {
                let msg = lock
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                Err(ServiceError::error(format!("[{}]", msg)))
            }
        }
    }
}

impl TaskExecutorFactory for ExecutorFactory {
    type Executor = Executor;

    fn new(ctx: TaskContext) -> (Self::Executor, SignalOutgoings) {
        Executor::new(ctx)
    }
}

async fn process<S, Ctx>(
    mut task_ctx: TaskContext,
    mut rx: Incoming<Frame>,
    upstream_count: u32,
    tx: Outgoing<Frame>,
    mut service: S,
    service_type: ServiceType,
    context: Ctx,
) -> Result<(), S::Error>
where
    S: Service<Request = Frame, Context = Ctx>,
{
    let service_starded_at = SystemTime::now();
    let tx_signal = tx.clone();
    let mut finish_count = 0;
    let mut sc = ServiceContext::Ready(context);

    let info_span = task_ctx.info_span();
    task_ctx.set_span(info_span.clone());

    let uri = task_ctx.uri();

    tracing::info!(
        parent: &info_span,
        ?uri,
        service=?service_type,
        operation=?Operation::StartService,
        "{}",
        Operation::StartService.as_message()
    );

    task_ctx
        .push_service_event(&uri, &service_type, MetricsEventKind::StartService)
        .await;

    sc = service.started(task_ctx.clone(), sc.into());

    //main loop, receive on message
    loop {
        let item = match sc {
            ServiceContext::Ready(_) => rx.next().await,
            ServiceContext::Complete(_) => None,
            ServiceContext::Next(_) => Some(Frame::default()),
        };

        task_ctx
            .push_service_event(&uri, &service_type, MetricsEventKind::ReceiveRequest)
            .await;

        match item {
            Some(req) if req.is_stop() => {
                tracing::info!(parent: &info_span, ?uri, "receive stop signal.");
                task_ctx
                    .push_service_event(&uri, &service_type, MetricsEventKind::ReceiveStop)
                    .await;
                break;
            }
            Some(req) if req.is_upstream_finish() => {
                finish_count += 1;
                tracing::info!(
                    parent: &info_span,
                    ?uri,
                    finish_count,
                    upstream_count,
                    "receive upstream finish signal."
                );
                task_ctx
                    .push_service_event(
                        &uri,
                        &service_type,
                        MetricsEventKind::ReceiveUpstreamFinish,
                    )
                    .await;
                {
                    let tx = tx.clone();
                    let ctx = task_ctx.clone();
                    sc = service.upstream_finish(ctx, sc.into(), req, tx).await?;
                    task_ctx
                        .push_service_event(
                            &uri,
                            &service_type,
                            MetricsEventKind::FinishUpstreamFinish,
                        )
                        .await;
                }
                if finish_count >= upstream_count {
                    tracing::info!(parent: &info_span, ?uri, "all upstream finish.");
                    let tx = tx.clone();
                    let ctx = task_ctx.clone();
                    sc = service.upstream_finish_all(ctx, sc.into(), tx).await?;
                    task_ctx
                        .push_service_event(
                            &uri,
                            &service_type,
                            MetricsEventKind::FinishUpstreamFinishAll,
                        )
                        .await;
                }
            }
            Some(req) => {
                let tx = tx.clone();
                sc = {
                    let task_ctx = task_ctx.clone();
                    service.handle(task_ctx, sc.into(), req, tx).await?
                };
                task_ctx
                    .push_service_event(&uri, &service_type, MetricsEventKind::SendRequest)
                    .await;
            }
            None => {
                on_finish(tx_signal, task_ctx.uri().clone()).await;
                break;
            }
        };
    }

    service.completed(task_ctx.clone(), sc.into());
    let _ = log_total_time(
        service_starded_at,
        &info_span,
        Some((&uri, &service_type)),
        Operation::FinishService,
    );
    task_ctx
        .push_service_event(&uri, &service_type, MetricsEventKind::FinishService)
        .await;
    Ok(())
}

async fn on_finish(mut tx: Outgoing<Frame>, uri: Uri) {
    let results = tx.send_ok_all(Frame::upstream_finish()).await;
    let errors = results
        .iter()
        .filter_map(|x| x.as_ref().err())
        .collect::<Vec<&OutgoingError>>();
    if errors.len() > 0 {
        tracing::error!(?uri, ?errors, "error, send upstream finish signal.",);
    }
}

fn log_total_time(
    started_at: SystemTime,
    parent: &Span,
    uri: Option<(&Uri, &ServiceType)>,
    operation: Operation,
) -> Option<f64> {
    let now = SystemTime::now();
    let ended = now.duration_since(started_at);
    if ended.is_ok() {
        let duration = ended.unwrap();
        let total = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
        match uri {
            Some((u, st)) => {
                tracing::info!(parent: parent, uri=?u, service=?st, total=?total, ?operation, "{}", operation.as_message());
            }
            None => {
                tracing::info!(parent: parent, total=?total, ?operation, "{}", operation.as_message());
            }
        };
        Some(total)
    } else {
        tracing::error!(parent: parent, ?operation, "error, calc total time.");
        None
    }
}

async fn push_error(
    errors: Arc<Mutex<Vec<ServiceError>>>,
    uri: &Uri,
    tp: &ServiceType,
    e: ServiceError,
) {
    tracing::error!(?uri, service=?tp, err = ?e, "{}", e);
    {
        let mut lock = errors.lock().await;
        lock.push(e);
    }
}
