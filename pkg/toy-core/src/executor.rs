use crate::data::{self, Frame};
use crate::error::ServiceError;
use crate::graph::{Graph, InputWire, OutputWire};
use crate::mpsc;
use crate::mpsc::{Incoming, Outgoing};
use crate::registry::Delegator;
use crate::service::{Service, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use log;
use std::collections::HashMap;
use std::future::Future;
use std::thread;
use toy_pack::deser::DeserializableOwned;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

pub trait AsyncRuntime {
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

pub struct Executor<'a, T>
where
    T: AsyncRuntime,
{
    rt: &'a T,
    starters: HashMap<Uri, Outgoing<Frame, ServiceError>>,
    awaiter: Incoming<Frame, ServiceError>,
    graph: Graph,
    inputs: HashMap<Uri, Incoming<Frame, ServiceError>>,
    outputs: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

impl<'a, T> Executor<'a, T>
where
    T: AsyncRuntime,
{
    pub fn new(rt: &'a T, graph: Graph) -> Self {
        let mut inputs: HashMap<Uri, Incoming<Frame, ServiceError>> = HashMap::new();
        let mut outputs: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let mut starters: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let (l_tx, l_rx) = mpsc::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);

        // first channel
        graph
            .inputs()
            .iter()
            .filter(|(_, w)| **w == InputWire::None)
            .for_each(|(uri, _)| {
                let (f_tx, f_rx) = mpsc::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
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
            let (tx, rx) = mpsc::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            match wire {
                InputWire::Single(o, i) => {
                    inputs.insert(i.clone(), rx);
                    outputs
                        .entry(o.clone())
                        .or_insert_with(|| Outgoing::empty())
                        .merge(tx);
                }
                InputWire::Fanin(o, i) => {
                    inputs.insert(i.clone(), rx);
                    o.iter().enumerate().for_each(|(i, x)| {
                        outputs
                            .entry(x.clone())
                            .or_insert_with(|| Outgoing::empty())
                            .merge_by(i as u8, tx.clone());
                    });
                }
                _ => (),
            }
        }

        Self {
            rt,
            starters,
            awaiter: l_rx,
            graph,
            inputs,
            outputs,
        }
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
        delegator: &impl Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
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
                log::error!("an error occured; {:?}", e);
                return Err(e);
            }
        }

        self.run_0(start_frame).await
    }

    async fn run_0(mut self, start_frame: Frame) -> Result<(), ServiceError> {
        log::info!("{:?} run executor", thread::current().id());

        for (_, mut tx) in self.starters {
            tx.send(Ok(start_frame.clone())).await?
        }

        while let Some(x) = self.awaiter.next().await {
            match x {
                Err(e) => log::error!("an error occured; {:?}", e),
                _ => (),
            }
        }

        Ok(())
    }
}

impl<'a, T> ServiceExecutor for Executor<'a, T>
where
    T: AsyncRuntime,
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
            log::error!("not found service. uri:{:?}", uri);
            return;
        }

        match data::unpack::<F::Config>(config_value.unwrap().config()) {
            Ok(config) => {
                self.rt.spawn(async move {
                    match factory.new_service(service_type.clone()).await {
                        Ok(service) => match factory.new_context(service_type.clone(), config) {
                            Ok(ctx) => {
                                if let Err(e) = process(rx, tx, service, ctx, &uri).await {
                                    log::error!("an error occured; uri:{:?}, error:{:?}", uri, e);
                                }
                            }
                            Err(e) => {
                                log::error!("an error occured; uri:{:?}, error:{:?}", uri, e);
                            }
                        },
                        Err(e) => {
                            log::error!("service initialize failed. uri:{:?}, error:{:?}", uri, e);
                        }
                    }
                });
            }
            Err(e) => log::error!("config initialize failed. uri:{:?}, error:{:?}", uri, e),
        }
    }
}

async fn process<S, Ctx>(
    mut rx: Incoming<Frame, ServiceError>,
    mut tx: Outgoing<Frame, ServiceError>,
    mut service: S,
    mut context: Ctx,
    uri: &Uri,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    log::info!("{:?} uri:{:?} start", thread::current().id(), uri);
    context = service.started(context);

    //main loop, receive on message
    while let Some(item) = rx.next().await {
        match item {
            Ok(req) => {
                let tx = tx.clone();
                context = service.handle(context, req, tx).await?;
            }
            Err(e) => {
                let _ = tx.send(Err(ServiceError::error(e))).await?;
            }
        };
    }

    let _ = service.completed(context);
    log::info!("{:?} uri:{:?} end", thread::current().id(), uri);

    Ok(())
}
