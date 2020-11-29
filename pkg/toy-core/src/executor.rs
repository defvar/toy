use crate::data::{self, Frame};
use crate::error::ServiceError;
use crate::graph::{Graph, InputWire, OutputWire};
use crate::mpsc;
use crate::mpsc::{Incoming, Outgoing};
use crate::registry::Delegator;
use crate::service::{Service, ServiceContext, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::collections::HashMap;
use std::future::Future;
use toy_pack::deser::DeserializableOwned;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

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
    starters: HashMap<Uri, Outgoing<Frame, ServiceError>>,
    awaiter: Incoming<Frame, ServiceError>,
    graph: Graph,
    inputs: HashMap<Uri, Incoming<Frame, ServiceError>>,
    outputs: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

impl<T> Executor<T>
where
    T: AsyncSpawner,
{
    pub fn new(spawner: T, graph: Graph) -> (Self, HashMap<Uri, Outgoing<Frame, ServiceError>>) {
        let mut inputs: HashMap<Uri, Incoming<Frame, ServiceError>> = HashMap::new();
        let mut outputs: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let mut starters: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let (l_tx, l_rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);

        // first channel
        graph
            .inputs()
            .iter()
            .filter(|(_, w)| **w == InputWire::None)
            .for_each(|(uri, _)| {
                let (f_tx, f_rx) =
                    mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
                inputs.insert(uri.clone(), f_rx);
                starters.insert(uri.clone(), f_tx);
            });

        // last channel
        graph
            .outputs()
            .iter()
            .filter(|(_, w)| **w == OutputWire::None)
            .for_each(|(uri, _)| {
                outputs
                    .entry(uri.clone())
                    .or_insert_with(|| Outgoing::empty())
                    .merge(l_tx.clone());
            });

        for (_, wire) in graph.inputs() {
            let (tx, rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            match wire {
                InputWire::Single(o, i) => {
                    inputs.insert(i.clone(), rx);
                    outputs
                        .entry(o.clone())
                        .or_insert_with(|| Outgoing::empty())
                        .merge(tx.clone());
                }
                InputWire::Fanin(o, i) => {
                    inputs.insert(i.clone(), rx);
                    o.iter().enumerate().for_each(|(idx, x)| {
                        outputs
                            .entry(x.clone())
                            .or_insert_with(|| Outgoing::empty())
                            .merge_by(idx as u8, tx.clone());
                    });
                }
                _ => (),
            }
        }

        let outputs_for_sv = {
            let mut map = HashMap::new();
            starters.iter().for_each(|(k, v)| {
                map.insert(k.clone(), v.clone());
            });
            outputs.iter().for_each(|(k, v)| {
                map.insert(k.clone(), v.clone());
            });
            map
        };

        (
            Self {
                spawner,
                starters,
                awaiter: l_rx,
                graph,
                inputs,
                outputs,
            },
            outputs_for_sv,
        )
    }

    fn pop_channels(
        &mut self,
        uri: &Uri,
    ) -> (Outgoing<Frame, ServiceError>, Incoming<Frame, ServiceError>) {
        if let Some(tx) = self.outputs.remove(uri) {
            if let Some(rx) = self.inputs.remove(uri) {
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

        for (_, tx) in &mut self.starters {
            tx.send(Ok(start_frame.clone())).await?
        }

        while let Some(x) = self.awaiter.next().await {
            match x {
                Err(e) => tracing::error!("an error occured, awaiter. {:?}", e),
                Ok(req) => {
                    if req.is_stop() {
                        tracing::info!("receive stop signal. awaiter.");
                        break;
                    }
                    if req.is_upstream_finish() {
                        tracing::info!("receive upstream finish signal. awaiter.");
                        break;
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
        let (tx, rx) = self.pop_channels(uri);
        let uri = uri.clone();
        let service_type = service_type.clone();

        let config_value = self.graph.by_uri(&uri);
        if config_value.is_none() {
            tracing::error!("not found service. uri:{:?}", uri);
            return;
        }

        let s = self.spawner.clone();
        match data::unpack::<F::Config>(config_value.unwrap().config()) {
            Ok(config) => {
                s.spawn(async move {
                    match factory.new_service(service_type.clone()).await {
                        Ok(service) => match factory.new_context(service_type.clone(), config) {
                            Ok(ctx) => {
                                if let Err(e) = process(rx, tx, service, ctx, &uri).await {
                                    tracing::error!(
                                        "an error occured; uri:{:?}, error:{:?}",
                                        uri,
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                tracing::error!("an error occured; uri:{:?}, error:{:?}", uri, e);
                            }
                        },
                        Err(e) => {
                            tracing::error!(
                                "service initialize failed. uri:{:?}, error:{:?}",
                                uri,
                                e
                            );
                        }
                    }
                });
            }
            Err(e) => tracing::error!("config initialize failed. uri:{:?}, error:{:?}", uri, e),
        }
    }
}

async fn process<S, Ctx>(
    mut rx: Incoming<Frame, ServiceError>,
    tx: Outgoing<Frame, ServiceError>,
    mut service: S,
    mut context: Ctx,
    uri: &Uri,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    tracing::info!("start node. uri:{:?}", uri);
    context = service.started(context);

    let tx2 = tx.clone();
    let mut force_next = false;

    //main loop, receive on message
    loop {
        let item = if force_next {
            Some(Ok(Frame::default()))
        } else {
            rx.next().await
        };

        match item {
            Some(Ok(req)) => {
                if req.is_stop() {
                    tracing::info!("receive stop signal. uri:{:?}", uri);
                    break;
                }
                if req.is_upstream_finish() {
                    tracing::info!("receive upstream finish signal. uri:{:?}", uri);
                    on_finish(tx2.clone(), uri).await;
                    break;
                }
                let tx = tx.clone();
                match service.handle(context, req, tx).await? {
                    ServiceContext::Complete(c) => {
                        context = c;
                        on_finish(tx2.clone(), uri).await;
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
                tracing::error!("node receive message error, :{:?}", e);
                return Err(e);
            }
            None => {
                break;
            }
        };
    }

    let _ = service.completed(context);
    tracing::info!("end node. uri:{:?}", uri);
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
