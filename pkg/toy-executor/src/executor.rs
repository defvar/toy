use async_trait::async_trait;
use toy_core::data::{self, Frame};
use toy_core::error::ServiceError;
use toy_core::executor::{ServiceExecutor, TaskExecutor, TaskExecutorFactory};
use toy_core::graph::Graph;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::node_channel::{self, Awaiter, Incomings, Outgoings, SignalOutgoings, Starters};
use toy_core::registry::Delegator;
use toy_core::service::{Service, ServiceContext, ServiceFactory};
use toy_core::task::TaskContext;
use toy_core::ServiceType;
use toy_core::Uri;
use toy_pack::deser::DeserializableOwned;

pub struct Executor {
    starters: Starters,
    awaiter: Awaiter,
    graph: Graph,
    incomings: Incomings,
    outgoings: Outgoings,
    ctx: TaskContext,
}

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
                    tracing::error!(parent: &span, "an error occured, awaiter. {:?}", e);
                }
                Ok(req) => {
                    if req.is_stop() {
                        tracing::info!(parent: &span, "receive stop signal. awaiter.");
                        break;
                    }
                    if req.is_upstream_finish() {
                        finish_count += 1;
                        tracing::info!(parent: &span, "receive upstream finish signal. awaiter, finish_count:{:?}, upstream_count:{:?}", finish_count, self.awaiter.upstream_count());
                        if finish_count >= self.awaiter.upstream_count() {
                            tracing::info!(parent: &span, "all upstream finish. awaiter.");
                            break;
                        }
                    }
                }
            }
        }

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
        F::Config: DeserializableOwned + Send,
    {
        let (tx, (rx, upstream_count)) = self.pop_channels(uri);
        let uri = uri.clone();
        let service_type = service_type.clone();

        let config_value = self.graph.by_uri(&uri);
        if config_value.is_none() {
            tracing::error!("not found service. uri:{:?}", uri);
            return;
        }

        let task_ctx = self.ctx.clone();
        let task_ctx = task_ctx.with_uri(&uri);
        match data::unpack::<F::Config>(config_value.unwrap().config()) {
            Ok(config) => {
                toy_rt::spawn(async move {
                    let new_service = factory.new_service(service_type.clone()).await;
                    let new_ctx = factory.new_context(service_type.clone(), config);
                    match (new_service, new_ctx) {
                        (Ok(service), Ok(ctx)) => {
                            if let Err(e) =
                                process(task_ctx, rx, upstream_count, tx, service, ctx).await
                            {
                                tracing::error!("an error occured; uri:{:?}, error:{:?}", uri, e);
                            }
                        }
                        (s, c) => {
                            let e1 = s.err();
                            let e2 = c.err();
                            tracing::error!("an error occured; uri:{:?}, service initialize error:{:?}. context initialize error:{:?}.", uri, e1, e2);
                        }
                    }
                });
            }
            Err(e) => tracing::error!("config initialize failed. uri:{:?}, error:{:?}", uri, e),
        }
    }
}

#[async_trait]
impl TaskExecutor for Executor {
    async fn run(
        mut self,
        delegator: impl Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>
            + Send
            + 'static,
        start_frame: Frame,
    ) -> Result<(), ServiceError> {
        let span = self.ctx.info_span();

        // need to reverse ....
        let nodes = self
            .graph
            .iter()
            .rev()
            .map(|x| (x.service_type(), x.uri()))
            .collect::<Vec<_>>();

        for (stype, uri) in &nodes {
            if let Err(e) = delegator.delegate(stype, uri, &mut self) {
                tracing::error!(parent: &span, "an error occured; {:?}", e);
                return Err(e);
            }
        }

        self.run_0(start_frame).await
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
    context: Ctx,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    let tx_signal = tx.clone();
    let mut finish_count = 0;
    let mut sc = ServiceContext::Ready(context);

    let info_span = task_ctx.info_span();
    task_ctx.set_span(info_span.clone());

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
                tracing::info!(parent: &info_span, "receive stop signal.");
                break;
            }
            Some(Ok(req)) if req.is_upstream_finish() => {
                finish_count += 1;
                tracing::info!(
                    parent: &info_span,
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
                    tracing::info!(parent: &info_span, "all upstream finish.");
                    let tx = tx.clone();
                    let task_ctx = task_ctx.clone();
                    sc = service.upstream_finish_all(task_ctx, sc.into(), tx).await?;
                }
            }
            Some(Ok(req)) => {
                let tx = tx.clone();
                let task_ctx = task_ctx.clone();
                sc = service.handle(task_ctx, sc.into(), req, tx).await?;
            }
            Some(Err(e)) => {
                tracing::error!(parent: &info_span, err=?e, "node receive message error");
                return Err(e);
            }
            None => {
                on_finish(tx_signal, task_ctx.uri().unwrap()).await;
                break;
            }
        };
    }

    service.completed(task_ctx.clone(), sc.into());
    tracing::info!(parent: &info_span, "end node.");
    Ok(())
}

async fn on_finish(mut tx: Outgoing<Frame, ServiceError>, uri: Uri) {
    let results = tx.send_ok_all(Frame::upstream_finish()).await;
    let results = results
        .iter()
        .filter_map(|x| x.as_ref().err())
        .collect::<Vec<&ServiceError>>();
    if results.len() > 0 {
        tracing::error!(
            "send upstream finish signal. uri:{:?}, ret:{:?}.",
            uri,
            results
        );
    }
}
