use crate::channel;
use crate::channel::{Incoming, Outgoing};
use crate::data::{self, Frame};
use crate::error::{Error, ServiceError};
use crate::graph::{Graph, InputWire, OutputWire};
use crate::registry::ServiceSpawner;
use crate::service::{Service, ServiceFactory};
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use futures::executor::ThreadPool;
use futures::future;
use log;
use std::collections::HashMap;
use std::thread;
use toy_pack::deser::DeserializableOwned;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

pub trait ServiceExecutor {
    type Request;
    type Error;
    type InitError;

    fn spawn<F>(&mut self, service_type: ServiceType, uri: Uri, factory: F)
    where
        F: ServiceFactory<
                Request = Self::Request,
                Error = Self::Error,
                InitError = Self::InitError,
            > + Send
            + Sync
            + 'static,
        F::Future: Send + 'static,
        F::Service: Send,
        <<F as ServiceFactory>::Service as Service>::Future: Send + 'static,
        F::Context: Send,
        F::Config: DeserializableOwned<Value = F::Config> + Send;
}

pub struct DefaultExecutor {
    pool: ThreadPool,
    starters: HashMap<Uri, Outgoing<Frame, ServiceError>>,
    awaiter: Incoming<Frame, ServiceError>,
    graph: Graph,
    inputs: HashMap<Uri, Incoming<Frame, ServiceError>>,
    outputs: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

impl DefaultExecutor {
    pub fn new(graph: Graph) -> Self {
        let mut inputs: HashMap<Uri, Incoming<Frame, ServiceError>> = HashMap::new();
        let mut outputs: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let mut starters: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

        let (l_tx, l_rx) = channel::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);

        graph
            .inputs()
            .iter()
            .filter(|(_, w)| **w == InputWire::None)
            .for_each(|(uri, _)| {
                let (f_tx, f_rx) =
                    channel::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
                inputs.insert(uri.clone(), f_rx);
                starters.insert(uri.clone(), f_tx);
            });

        if let Some((uri, _)) = graph
            .outputs()
            .iter()
            .find(|(_, w)| **w == OutputWire::None)
        {
            outputs.insert(uri.clone(), l_tx);
        }

        for (_, wire) in graph.inputs() {
            let (tx, rx) = channel::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            match wire {
                InputWire::Single(o, i) => {
                    inputs.insert(i.clone(), rx);
                    outputs.insert(o.clone(), tx);
                }
                InputWire::Fanin(o, i) => {
                    inputs.insert(i.clone(), rx);
                    o.iter().for_each(|x| {
                        outputs.insert(x.clone(), tx.clone());
                    });
                }
                _ => (),
            }
        }

        Self {
            pool: ThreadPool::new().unwrap(),
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
        spawner: impl ServiceSpawner<ServiceExecutor = Self>,
        start_frame: Frame,
    ) -> Result<(), ServiceError> {
        // need to reverse ....
        let nodes = self
            .graph
            .iter()
            .rev()
            .map(|x| (x.service_type().clone(), x.uri().clone()))
            .collect::<Vec<_>>();

        for (stype, uri) in nodes {
            let _ = spawner.spawn(stype, uri, &mut self);
        }

        self.run_0(start_frame).await
    }

    async fn run_0(self, start_frame: Frame) -> Result<(), ServiceError> {
        log::info!("{:?} start flow", thread::current().id());

        for (_, mut tx) in self.starters {
            tx.send(Ok(start_frame.clone())).await?
        }

        self.awaiter
            .for_each(|x| {
                match x {
                    Err(e) => {
                        log::error!("error {:?}", e);
                    }
                    _ => (),
                };
                future::ready(())
            })
            .await;

        Ok(())
    }
}

impl ServiceExecutor for DefaultExecutor {
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn spawn<F>(&mut self, service_type: ServiceType, uri: Uri, factory: F)
    where
        F: ServiceFactory<
                Request = Self::Request,
                Error = Self::Error,
                InitError = Self::InitError,
            > + Send
            + Sync
            + 'static,
        F::Future: Send + 'static,
        F::Service: Send,
        <<F as ServiceFactory>::Service as Service>::Future: Send + 'static,
        F::Context: Send,
        F::Config: DeserializableOwned<Value = F::Config> + Send,
    {
        let (tx, rx) = self.pop_channels(&uri);
        let service_type = service_type.clone();
        if let Some(config_value) = self.graph.by_uri(&uri) {
            match data::unpack::<F::Config>(config_value.config()) {
                Ok(config) => {
                    self.pool.spawn_ok(async move {
                        match factory.new_service(service_type.clone()).await {
                            Ok(service) => {
                                let ctx = factory.new_context(service_type.clone(), config);
                                if let Err(e) = process(rx, tx, service, ctx).await {
                                    log::error!("an error occured; error = {:?}", e);
                                }
                            }
                            Err(e) => {
                                log::error!("service initialize error = {:?}", e);
                            }
                        }
                    });
                }
                Err(e) => log::error!("config initialize error = {:?}", e),
            }
        }
    }
}

async fn process<S, Ctx>(
    mut rx: Incoming<Frame, ServiceError>,
    mut tx: Outgoing<Frame, ServiceError>,
    mut service: S,
    mut context: Ctx,
) -> Result<(), ServiceError>
where
    S: Service<Request = Frame, Error = ServiceError, Context = Ctx>,
{
    log::info!("{:?} start serivce", thread::current().id());
    context = service.started(context);

    //main loop, receive on message
    while let Some(item) = rx.next().await {
        match item {
            Ok(req) => {
                let tx = tx.clone();
                context = service.handle(context, req, tx).await?;
            }
            Err(e) => {
                let _ = tx.send(Err(Error::custom(e))).await?;
            }
        };
    }

    let _ = service.completed(context);
    log::info!("{:?} end serivce", thread::current().id());

    Ok(())
}
