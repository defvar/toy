use super::channel;
use super::data::Frame;
use super::error::MessagingError;
use crate::channel::{Incoming, Outgoing};
use futures::executor::ThreadPool;
use futures::future;
use log::{error, info};
use std::thread;

type R = Result<Frame, MessagingError>;

enum SenderOrReceiver<T> {
    Sender(Outgoing<T>),
    Receiver(Incoming<T>),
}

impl<T> SenderOrReceiver<T> {
    fn sender(self) -> Option<Outgoing<T>> {
        match self {
            SenderOrReceiver::Sender(x) => Some(x),
            SenderOrReceiver::Receiver(_) => None,
        }
    }

    fn receiver(self) -> Option<Incoming<T>> {
        match self {
            SenderOrReceiver::Sender(_) => None,
            SenderOrReceiver::Receiver(x) => Some(x),
        }
    }
}

pub struct Flow {
    pool: ThreadPool,
    service_names: Vec<String>,
    channel_buffer_size: usize,
}

impl Flow {
    pub fn new(service_names: Vec<String>) -> Flow {
        Flow {
            pool: ThreadPool::new().unwrap(),
            service_names,
            channel_buffer_size: 12,
        }
    }

    pub async fn start(&self) -> Result<(), MessagingError> {
        if self.service_names.len() == 0 {
            return Ok(());
        }

        let mut channels: Vec<SenderOrReceiver<R>> = Vec::new();

        // index:0 -> first tx
        // index:1 -> service:0, rx
        // index:2 -> service:0, tx
        // ...
        // index:n   -> service:n-1, rx
        // index:n+1 -> service:n-1, tx
        // ...
        // index:last -> last rx

        for _ in 0..self.service_names.len() + 1 {
            let (tx, rx) = channel::stream::<R>(self.channel_buffer_size);
            channels.push(SenderOrReceiver::Sender(tx));
            channels.push(SenderOrReceiver::Receiver(rx));
        }

        let last_rx = channels.pop();
        let mut tail = channels.split_off(1);
        let first_tx = channels.pop();

        for service_name in self.service_names.iter().rev() {
            let service_name = service_name.to_string();

            if let Some(tx) = tail.pop().map(|x| x.sender()).flatten() {
                if let Some(rx) = tail.pop().map(|x| x.receiver()).flatten() {
                    // start thread
                    self.pool.spawn_ok(async move {
                        if let Err(e) = Flow::process0(rx, tx, service_name).await {
                            error!("an error occured; error = {:?}", e);
                        }
                        info!("{:?} spawn task end", thread::current().id())
                    });
                }
            }
        }

        if let Some(item) = first_tx {
            if let Some(mut tx) = item.sender() {
                tx.send(Ok(Frame::none())).await?;
            }
        }

        if let Some(item) = last_rx {
            if let Some(rx) = item.receiver() {
                rx.for_each(|x| {
                    match x {
                        Ok(r) => {
                            info!("receive {:?}", r);
                        }
                        Err(e) => {
                            error!("error {:?}", e);
                        }
                    };
                    future::ready(())
                })
                .await;
            }
        }

        info!("flow end");

        Ok(())
    }

    async fn process0(
        mut rx: Incoming<R>,
        mut tx: Outgoing<R>,
        service_name: String,
    ) -> Result<(), MessagingError> {
        //
        //prepare handler and context
        //

        info!(
            "{:?} start serivce {:?}",
            thread::current().id(),
            service_name
        );

        //main loop, receive on message
        while let Some(item) = rx.next().await {
            match item {
                Ok(req) => {
                    info!(
                        "{:?} serivce:{:?} receive {:?}",
                        thread::current().id(),
                        service_name,
                        req
                    );
                    //                    let _ = h.handle(&mut c, req);
                }
                Err(e) => {
                    error!("error {:?}", e);
                    let _ = tx.send(Err(MessagingError::error(e))).await?;
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
