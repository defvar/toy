use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::{AsyncRuntime, Executor};
use crate::graph::Graph;
use crate::mpsc::{self, Incoming, Outgoing};
use crate::oneshot;
use crate::registry::{Delegator, ServiceSet};
use crate::service::ServiceFactory;
use crate::ServiceType;
use toy_pack::deser::DeserializableOwned;

#[derive(Debug)]
pub enum Request {
    Task(Graph),
    Services(oneshot::Outgoing<Response, ServiceError>),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    Services(Vec<ServiceType>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemMessage {
    Shutdown,
}

#[derive(Debug)]
pub struct Supervisor<T, S, F> {
    rt: T,
    registry: ServiceSet<S, F>,

    /// send system message to api server
    tx: Outgoing<SystemMessage, ServiceError>,

    /// receive any request from api server
    rx: Incoming<Request, ServiceError>,
}

impl<T, S, F, R> Supervisor<T, S, F>
where
    T: AsyncRuntime,
    S: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError>,
    F: Fn() -> R + Clone,
    R: ServiceFactory<Request = Frame, Error = ServiceError, InitError = ServiceError>
        + Send
        + Sync
        + 'static,
    R::Service: Send,
    R::Context: Send,
    R::Config: DeserializableOwned<Value = R::Config> + Send,
{
    pub fn new(
        rt: T,
        registry: ServiceSet<S, F>,
    ) -> (
        Supervisor<T, S, F>,
        Outgoing<Request, ServiceError>,
        Incoming<SystemMessage, ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::stream::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::stream::<SystemMessage, ServiceError>(1024);
        (
            Supervisor {
                rt,
                registry,
                tx: tx_sys,
                rx: rx_req,
            },
            tx_req,
            rx_sys,
        )
    }

    pub async fn run(mut self) -> Result<(), ()> {
        log::info!("start supervisor");
        while let Some(r) = self.rx.next().await {
            match r {
                Ok(m) => match m {
                    Request::Task(g) => {
                        let e = Executor::new(&self.rt, g);
                        let _ = e.run(&self.registry, Frame::default()).await;
                        log::info!("executed task");
                    }
                    Request::Services(tx) => {
                        let m = Response::Services(self.registry.service_types().clone());
                        let _ = tx.send_ok(m).await;
                    }
                    Request::Shutdown => {
                        log::info!("supervisor, receive shutdown request.");
                        break;
                    }
                },
                Err(e) => log::error!("an error occured; supervisor, error:{:?}", e),
            }
        }
        log::info!("shutdown supervisor");
        let _ = self.tx.send_ok(SystemMessage::Shutdown).await;
        Ok(())
    }
}
