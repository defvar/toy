use crate::data::{self, Frame};
use crate::error::ServiceError;
use crate::graph::Graph;
use crate::mpsc::{Incoming, Outgoing};
use crate::node_channel::{self, Awaiter, Incomings, Outgoings, SignalOutgoings, Starters};
use crate::registry::Delegator;
use crate::service::{Service, ServiceContext, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use crate::task::TaskContext;
use std::future::Future;
use toy_pack::deser::DeserializableOwned;

pub trait AsyncSpawner: Clone + Send {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

pub trait ServiceExecutor {
    type Request;
    type Error;
    type InitError;

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
        F::Config: DeserializableOwned + Send;
}

pub struct Executor<T> {
    spawner: T,
    starters: Starters,
    awaiter: Awaiter,
    graph: Graph,
    incomings: Incomings,
    outgoings: Outgoings,
    ctx: TaskContext,
}

impl<T> Executor<T>
where
    T: AsyncSpawner,
{
    pub fn new(spawner: T, ctx: TaskContext) -> (Self, SignalOutgoings) {
        let graph = ctx.graph().clone();
        let (incomings, outgoings, awaiter, starters, signals) = node_channel::from_graph(&graph);
        (
            Self {
                spawner,
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

    pub async fn run(
        mut self,
        delegator: impl Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
        start_frame: Frame,
    ) -> Result<(), ServiceError> {
        // need to reverse ....
        let nodes = self
            .graph
            .iter()
            .rev()
            .map(|x| (x.service_type(), x.uri()))
            .collect::<Vec<_>>();

        for (stype, uri) in &nodes {
            if let Err(e) = delegator.delegate(stype, uri, &mut self) {
                tracing::error!("an error occured; {:?}", e);
                return Err(e);
            }
        }

        self.run_0(start_frame).await
    }

    async fn run_0(mut self, start_frame: Frame) -> Result<(), ServiceError> {
        tracing::info!("run executor");

        for (_, tx) in &mut self.starters.iter_mut() {
            tx.send(Ok(start_frame.clone())).await?
        }

        let mut finish_count = 0;
        while let Some(x) = self.awaiter.next().await {
            match x {
                Err(e) => tracing::error!("an error occured, awaiter. {:?}", e),
                Ok(req) => {
                    if req.is_stop() {
                        tracing::info!("receive stop signal. awaiter.");
                        break;
                    }
                    if req.is_upstream_finish() {
                        finish_count += 1;
                        tracing::info!("receive upstream finish signal. awaiter, finish_count:{:?}, upstream_count:{:?}", finish_count, self.awaiter.upstream_count());
                        if finish_count >= self.awaiter.upstream_count() {
                            tracing::info!("all upstream finish. awaiter.");
                            break;
                        }
                    }
                }
            }
        }

        tracing::info!("end executor");

        Ok(())
    }
}

impl<T> ServiceExecutor for Executor<T>
where
    T: AsyncSpawner,
{
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

        let s = self.spawner.clone();
        let task_ctx = self.ctx.clone();
        match data::unpack::<F::Config>(config_value.unwrap().config()) {
            Ok(config) => {
                s.spawn(async move {
                    let new_service = factory.new_service(service_type.clone()).await;
                    let new_ctx = factory.new_context(service_type.clone(), config);
                    match (new_service, new_ctx) {
                        (Ok(service), Ok(ctx)) => {
                            if let Err(e) =
                                process(task_ctx, rx, upstream_count, tx, service, ctx, &uri).await
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

async fn process<S, Ctx>(
    task_ctx: TaskContext,
    mut rx: Incoming<Frame, ServiceError>,
    upstream_count: u32,
    tx: Outgoing<Frame, ServiceError>,
    mut service: S,
    mut context: Ctx,
    uri: &Uri,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    tracing::info!(uuid = ?task_ctx.uuid(), uri = ?uri, "start node.");
    context = service.started(task_ctx.clone(), context);

    let tx_signal = tx.clone();
    let mut force_next = false;
    let mut finish_count = 0;

    //main loop, receive on message
    loop {
        let item = if force_next {
            Some(Ok(Frame::default()))
        } else {
            rx.next().await
        };

        match item {
            Some(Ok(req)) if req.is_stop() => {
                tracing::info!(
                    uuid = ?task_ctx.uuid(),
                    uri = ?uri,
                    "receive stop signal.",
                );
                break;
            }
            Some(Ok(req)) if req.is_upstream_finish() => {
                finish_count += 1;
                tracing::info!(
                    uri = ?uri,
                    finish_count,
                    upstream_count,
                    "receive upstream finish signal."
                );
                if finish_count >= upstream_count {
                    tracing::info!(uri = ?uri, "all upstream finish.");
                    on_finish(tx_signal, uri).await;
                    break;
                }
            }
            Some(Ok(req)) => {
                let tx = tx.clone();
                let task_ctx = task_ctx.clone();
                match service.handle(task_ctx, context, req, tx).await? {
                    ServiceContext::Complete(c) => {
                        context = c;
                        on_finish(tx_signal, uri).await;
                        break;
                    }
                    ServiceContext::Ready(c) => {
                        context = c;
                        force_next = false;
                    }
                    ServiceContext::Next(c) => {
                        context = c;
                        force_next = true;
                    }
                }
            }
            Some(Err(e)) => {
                tracing::error!(err=?e, "node receive message error");
                return Err(e);
            }
            None => {
                break;
            }
        };
    }

    let _ = service.completed(task_ctx.clone(), context);
    tracing::info!(uuid = ?task_ctx.uuid(), uri = ?uri, "end node.");
    Ok(())
}

async fn on_finish(mut tx: Outgoing<Frame, ServiceError>, uri: &Uri) {
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
