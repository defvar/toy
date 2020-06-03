use crate::data::Frame;
use crate::error::ServiceError;
use crate::executor::{AsyncRuntime, Executor};
use crate::graph::Graph;
use crate::mpsc::{self, Incoming, Outgoing};
use crate::oneshot;
use crate::registry::{App, Delegator, Registry, ServiceSchema};

#[derive(Debug)]
pub enum Request {
    Task(Graph),
    Services(oneshot::Outgoing<Response, ServiceError>),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    Services(Vec<ServiceSchema>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemMessage {
    Shutdown,
}

#[derive(Debug)]
pub struct Supervisor<T, O, P> {
    service_rt: T,
    app: App<O, P>,

    /// send system message to api server.
    tx: Outgoing<SystemMessage, ServiceError>,

    /// receive any request from api server.
    rx: Incoming<Request, ServiceError>,
}

impl<T, O, P> Supervisor<T, O, P>
where
    T: AsyncRuntime,
    O: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError> + Registry,
    P: Delegator<Request = Frame, Error = ServiceError, InitError = ServiceError> + Registry,
{
    pub fn new(
        service_rt: T,
        registry: App<O, P>,
    ) -> (
        Supervisor<T, O, P>,
        Outgoing<Request, ServiceError>,
        Incoming<SystemMessage, ServiceError>,
    ) {
        let (tx_req, rx_req) = mpsc::stream::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::stream::<SystemMessage, ServiceError>(1024);
        (
            Supervisor {
                service_rt,
                app: registry,
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
                        let e = Executor::new(&self.service_rt, g);
                        let _ = e.run(&self.app, Frame::default()).await;
                        log::info!("executed task");
                    }
                    Request::Services(tx) => {
                        let m = Response::Services(self.app.schemas());
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
