use crate::channel;
use crate::channel::{Incoming, Outgoing};
use crate::data::Frame;
use crate::error::{Error, ServiceError};
use crate::service::{Service, ServiceFactory};
use crate::service_id::ServiceId;
use futures::executor::ThreadPool;
use futures::future;
use log;
use std::thread;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

pub trait ServiceExecutor {
    type Request;
    type Error;
    type InitError;

    fn spawn<F>(&mut self, service_id: ServiceId, factory: F)
    where
        F: ServiceFactory<
                Request = Self::Request,
                Error = Self::Error,
                InitError = Self::InitError,
            > + Send
            + Sync
            + 'static,
        F::Future: Send + 'static,
        <F as ServiceFactory>::Service: Send,
        <<F as ServiceFactory>::Service as Service>::Future: Send + 'static,
        F::Context: Send;
}

pub struct DefaultExecutor {
    pool: ThreadPool,
    channels: Vec<SenderOrReceiver<Frame, ServiceError>>,
    starter: Outgoing<Frame, ServiceError>,
    awaiter: Incoming<Frame, ServiceError>,
}

impl DefaultExecutor {
    pub fn new(service_names: Vec<String>) -> Self {
        let mut channels: Vec<SenderOrReceiver<Frame, ServiceError>> = Vec::new();

        for _ in 0..service_names.len() + 1 {
            let (tx, rx) = channel::stream::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            channels.push(SenderOrReceiver::Sender(tx));
            channels.push(SenderOrReceiver::Receiver(rx));
        }

        let last_rx = channels.pop();
        let tail = channels.split_off(1);
        let first_tx = channels.pop();

        Self {
            pool: ThreadPool::new().unwrap(),
            channels: tail,
            starter: first_tx.map(|x| x.sender().unwrap()).unwrap(),
            awaiter: last_rx.map(|x| x.receiver().unwrap()).unwrap(),
        }
    }

    pub async fn run(mut self, start_frame: Frame) -> Result<(), ServiceError> {
        log::info!("{:?} start flow", thread::current().id());

        self.starter.send(Ok(start_frame)).await?;
        drop(self.starter);

        log::info!("{:?} wait....", thread::current().id());
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

    fn spawn<F>(&mut self, service_id: ServiceId, factory: F)
    where
        F: ServiceFactory<
                Request = Self::Request,
                Error = Self::Error,
                InitError = Self::InitError,
            > + Send
            + Sync
            + 'static,
        F::Future: Send + 'static,
        <F as ServiceFactory>::Service: Send,
        <<F as ServiceFactory>::Service as Service>::Future: Send + 'static,
        F::Context: Send,
    {
        if let Some(tx) = self.channels.pop().map(|x| x.sender()).flatten() {
            if let Some(rx) = self.channels.pop().map(|x| x.receiver()).flatten() {
                let service_id = service_id.clone();
                self.pool.spawn_ok(async move {
                    match factory.new_service(service_id.clone()).await {
                        Ok(service) => {
                            let ctx = factory.new_context(service_id);
                            if let Err(e) = process(rx, tx, service, ctx).await {
                                log::error!("an error occured; error = {:?}", e);
                            }
                            log::info!("{:?} spawn task end", thread::current().id())
                        }
                        Err(e) => {
                            log::error!("service initialize error = {:?}", e);
                        }
                    }
                });
            }
        }
    }
}

enum SenderOrReceiver<T, Err> {
    Sender(Outgoing<T, Err>),
    Receiver(Incoming<T, Err>),
}

impl<T, Err> SenderOrReceiver<T, Err> {
    fn sender(self) -> Option<Outgoing<T, Err>> {
        match self {
            SenderOrReceiver::Sender(x) => Some(x),
            SenderOrReceiver::Receiver(_) => None,
        }
    }

    fn receiver(self) -> Option<Incoming<T, Err>> {
        match self {
            SenderOrReceiver::Sender(_) => None,
            SenderOrReceiver::Receiver(x) => Some(x),
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
    log::info!("{:?} end serivce", thread::current().id());

    Ok(())
}
