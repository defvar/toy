use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::time::{SystemTime, UNIX_EPOCH};
use toy_core::data::{self, Frame};
use toy_core::error::ServiceError;
use toy_core::executor::{ServiceExecutor, TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::metrics::EventRecord;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::node_channel::{self, Awaiter, Incomings, Outgoings, SignalOutgoings, Starters};
use toy_core::registry::{App, Registry};
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
            },
            signals,
        )
    }

    fn pop_channels(
        &mut self,
        uri: &Uri,
    ) -> (
        Outgoing<Frame, ServiceError>,
        (Incoming<Frame, ServiceError>, u32),
    ) {
        if let Some(tx) = self.outgoings.pop(uri) {
            if let Some(rx) = self.incomings.pop(uri) {
                return (tx, rx);
            }
        }
        panic!("invalid channel stack...");
    }

    async fn run_0(mut self, start_frame: Frame) -> Result<(), ServiceError> {
        for (_, tx) in &mut self.starters.iter_mut() {
            tx.send(Ok(start_frame.clone())).await?
        }

        let uri: Uri = "awaiter".into();
        let awaiter_ctx = self.ctx.with_uri(&uri);
        let span = awaiter_ctx.info_span();
        let mut finish_count = 0;
        while let Some(x) = self.awaiter.next().await {
            match x {
                Err(e) => {
                    tracing::error!(parent: &span, ?uri, err=?e, "an error occured.");
                }
                Ok(req) => {
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
            }
        }
        awaiter_ctx
            .push_event(EventRecord::task_event(
                awaiter_ctx.id(),
                Operation::FinishTask,
                now(),
            ))
            .await;
        Ok(())
    }
}

impl ServiceExecutor for Executor {
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn spawn<F>(&mut self, service_type: &ServiceType, uri: &Uri, factory: F)
    where
        F: ServiceFactory<
                Request = Self::Request,
                Error = Self::Error,
                InitError = Self::InitError,
            > + Send
            + Sync
            + 'static,
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
                                    service_type,
                                    ctx,
                                )
                                .await
                                {
                                    tracing::error!(?uri, err=?e, "an error occured;");
                                }
                            }
                            (s, c) => {
                                let e1 = s.err();
                                let e2 = c.err();
                                tracing::error!(?uri, err1 = ?e1, err2 = ?e2, "an error occured;");
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
    async fn run<T>(mut self, app: &App<T>, start_frame: Frame) -> Result<(), ServiceError>
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
        self.ctx
            .push_event(EventRecord::task_event(
                self.ctx.id(),
                Operation::StartTask,
                now(),
            ))
            .await;
        // need to reverse ....
        let nodes = self
            .graph
            .iter()
            .rev()
            .map(|x| (x.service_type(), x.uri()))
            .collect::<Vec<_>>();

        for (stype, uri) in &nodes {
            if let Err(e) = app.delegate(stype, uri, &mut self) {
                tracing::error!(parent: &span, err=?e, "an error occured;");
                return Err(e);
            }
        }

        let r = self.run_0(start_frame).await;
        log_total_time(started_at, &span, None, Operation::FinishTask);
        r
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
    mut rx: Incoming<Frame, ServiceError>,
    upstream_count: u32,
    tx: Outgoing<Frame, ServiceError>,
    mut service: S,
    service_type: ServiceType,
    context: Ctx,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    let service_starded_at = SystemTime::now();
    let tx_signal = tx.clone();
    let mut finish_count = 0;
    let mut sc = ServiceContext::Ready(context);

    let info_span = task_ctx.info_span();
    task_ctx.set_span(info_span.clone());

    let uri = task_ctx.uri().unwrap();

    tracing::info!(
        parent: &info_span,
        ?uri,
        service=?service_type,
        operation=?Operation::StartService,
        "{}",
        Operation::StartService.as_message()
    );

    task_ctx
        .push_event(EventRecord::service_event(
            task_ctx.id(),
            &uri,
            Operation::StartService,
            now(),
        ))
        .await;
    task_ctx.metrics().inc_service_start_count();

    sc = service.started(task_ctx.clone(), sc.into());

    //main loop, receive on message
    loop {
        let item = match sc {
            ServiceContext::Ready(_) => rx.next().await,
            ServiceContext::Complete(_) => None,
            ServiceContext::Next(_) => Some(Ok(Frame::default())),
        };

        match item {
            Some(Ok(req)) if req.is_stop() => {
                tracing::info!(parent: &info_span, ?uri, "receive stop signal.");
                break;
            }
            Some(Ok(req)) if req.is_upstream_finish() => {
                finish_count += 1;
                tracing::info!(
                    parent: &info_span,
                    ?uri,
                    finish_count,
                    upstream_count,
                    "receive upstream finish signal."
                );
                {
                    let tx = tx.clone();
                    let task_ctx = task_ctx.clone();
                    sc = service
                        .upstream_finish(task_ctx, sc.into(), req, tx)
                        .await?;
                }
                if finish_count >= upstream_count {
                    tracing::info!(parent: &info_span, ?uri, "all upstream finish.");
                    let tx = tx.clone();
                    let task_ctx = task_ctx.clone();
                    sc = service.upstream_finish_all(task_ctx, sc.into(), tx).await?;
                }
            }
            Some(Ok(req)) => {
                task_ctx.metrics().inc_request_receive_count();
                let tx = tx.clone();
                sc = {
                    let task_ctx = task_ctx.clone();
                    task_ctx.metrics().inc_request_send_count();
                    service.handle(task_ctx, sc.into(), req, tx).await?
                };
                task_ctx.metrics().inc_request_complete_count();
            }
            Some(Err(e)) => {
                tracing::error!(parent: &info_span, ?uri, err=?e, "node receive message error");
                return Err(e);
            }
            None => {
                on_finish(tx_signal, task_ctx.uri().unwrap()).await;
                break;
            }
        };
    }

    service.completed(task_ctx.clone(), sc.into());
    let _ = log_total_time(
        service_starded_at,
        &info_span,
        Some((&uri, service_type)),
        Operation::FinishService,
    );
    task_ctx
        .push_event(EventRecord::service_event(
            task_ctx.id(),
            &uri,
            Operation::FinishService,
            now(),
        ))
        .await;
    task_ctx.metrics().inc_service_finish_count();
    Ok(())
}

async fn on_finish(mut tx: Outgoing<Frame, ServiceError>, uri: Uri) {
    let results = tx.send_ok_all(Frame::upstream_finish()).await;
    let results = results
        .iter()
        .filter_map(|x| x.as_ref().err())
        .collect::<Vec<&ServiceError>>();
    if results.len() > 0 {
        tracing::error!(?uri, ?results, "send upstream finish signal.",);
    }
}

fn log_total_time(
    started_at: SystemTime,
    parent: &Span,
    uri: Option<(&Uri, ServiceType)>,
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

fn now() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.as_secs_f64())
        .ok()
}
