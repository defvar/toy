use super::channel;
use crate::channel::{Incoming, Outgoing};
use crate::context_box::BoxContext;
use crate::error::Error;
use crate::registry::Registry;
use crate::service_box::BoxService;
use futures::executor::ThreadPool;
use futures::future;
use log::{error, info};
use std::thread;

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

pub struct Flow {
    pool: ThreadPool,
    channel_buffer_size: usize,
}

impl Flow {
    pub fn new() -> Flow {
        Flow {
            pool: ThreadPool::new().unwrap(),
            channel_buffer_size: 12,
        }
    }

    pub async fn start<Req, Err, InitErr>(
        &self,
        registry: Registry<Req, Err, InitErr>,
        service_names: Vec<String>,
    ) -> Result<(), Err>
    where
        Req: Send + Default + 'static,
        Err: Error,
        InitErr: Error,
    {
        if service_names.len() == 0 {
            return Ok(());
        }

        let mut channels: Vec<SenderOrReceiver<Req, Err>> = Vec::new();

        // index:0 -> first tx
        // index:1 -> service:0, rx
        // index:2 -> service:0, tx
        // ...
        // index:n   -> service:n-1, rx
        // index:n+1 -> service:n-1, tx
        // ...
        // index:last -> last rx

        for _ in 0..service_names.len() + 1 {
            let (tx, rx) = channel::stream::<Req, Err>(self.channel_buffer_size);
            channels.push(SenderOrReceiver::Sender(tx));
            channels.push(SenderOrReceiver::Receiver(rx));
        }

        let last_rx = channels.pop();
        let mut tail = channels.split_off(1);
        let first_tx = channels.pop();

        for service_name in service_names.iter().rev() {
            if let Some(factory) = registry.get(service_name) {
                if let Some(tx) = tail.pop().map(|x| x.sender()).flatten() {
                    if let Some(rx) = tail.pop().map(|x| x.receiver()).flatten() {
                        let service_name = service_name.to_owned();
                        match factory.0.new_handler().await {
                            Ok(service) => {
                                let c = factory.1.new_context();
                                // start new thread
                                self.pool.spawn_ok(async move {
                                    if let Err(e) =
                                        Flow::process0(rx, tx, service_name, service, c).await
                                    {
                                        error!("an error occured; error = {:?}", e);
                                    }
                                    info!("{:?} spawn task end", thread::current().id())
                                });
                            }
                            Err(e) => {
                                error!("service init error = {:?}", e);
                            }
                        }
                    }
                }
            } else {
                error!("service not found = {:?}", service_name);
            }
        }

        if let Some(item) = first_tx {
            if let Some(mut tx) = item.sender() {
                tx.send(Ok(Req::default())).await?;
            }
        }

        if let Some(item) = last_rx {
            if let Some(rx) = item.receiver() {
                rx.for_each(|x| {
                    match x {
                        Err(e) => {
                            error!("error {:?}", e);
                        }
                        _ => (),
                    };
                    future::ready(())
                })
                .await;
            }
        }

        info!("flow end");

        Ok(())
    }

    async fn process0<Req, Err>(
        mut rx: Incoming<Req, Err>,
        mut tx: Outgoing<Req, Err>,
        service_name: String,
        mut service: BoxService<Req, Err>,
        mut context: BoxContext,
    ) -> Result<(), Err>
    where
        Err: Error,
    {
        info!(
            "{:?} start serivce {:?}",
            thread::current().id(),
            service_name
        );

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
        info!(
            "{:?} end serivce:{:?}",
            thread::current().id(),
            service_name
        );

        Ok(())
    }
}
