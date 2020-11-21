use crate::data::{Frame, Value};
use crate::error::ServiceError;
use crate::executor::{AsyncRuntime, Executor};
use crate::graph::Graph;
use crate::mpsc::{self, Incoming, Outgoing, OutgoingMessage};
use crate::oneshot;
use crate::registry::{App, Delegator, Registry, ServiceSchema};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub enum Request {
    Task(Graph),
    Stop(Uuid, oneshot::Outgoing<(), ServiceError>),
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

impl OutgoingMessage for Request {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for Response {
    fn set_port(&mut self, _port: u8) {}
}

impl OutgoingMessage for SystemMessage {
    fn set_port(&mut self, _port: u8) {}
}

#[derive(Debug)]
pub struct RunningGraph {
    started_at: Duration,
    config: Value,
    /// send stop message to graphs.
    tx: broadcast::Sender<()>,
}

#[derive(Debug)]
pub struct Supervisor<T, O, P> {
    service_rt: T,
    app: App<O, P>,

    /// send system message to api server.
    tx: Outgoing<SystemMessage, ServiceError>,

    /// receive any request from api server.
    rx: Incoming<Request, ServiceError>,

    graphs: Arc<Mutex<HashMap<Uuid, RunningGraph>>>,
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
        let (tx_req, rx_req) = mpsc::channel::<Request, ServiceError>(1024);
        let (tx_sys, rx_sys) = mpsc::channel::<SystemMessage, ServiceError>(1024);
        (
            Supervisor {
                service_rt,
                app: registry,
                tx: tx_sys,
                rx: rx_req,
                graphs: Arc::new(Mutex::new(HashMap::new())),
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
                        let (tx, _) = broadcast::channel(16);
                        let config = g.config().clone();
                        let e = Executor::new(&self.service_rt, g, &tx);
                        let running = RunningGraph {
                            started_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards"),
                            config,
                            tx,
                        };
                        let uuid = Uuid::new_v4();
                        {
                            let graphs = Arc::clone(&self.graphs);
                            let mut graphs = graphs.lock().await;
                            let _ = graphs.insert(uuid, running);
                        }
                        let _ = e.run(&self.app, Frame::default()).await;
                        {
                            let graphs = Arc::clone(&self.graphs);
                            let mut graphs = graphs.lock().await;
                            let _ = graphs.remove(&uuid);
                        }
                        log::info!("executed task");
                    }
                    Request::Stop(uuid, tx) => {
                        {
                            let graphs = Arc::clone(&self.graphs);
                            let mut graphs = graphs.lock().await;
                            if let Some(running) = graphs.get_mut(&uuid) {
                                running.tx.send(()).unwrap();
                                let _ = graphs.remove(&uuid);
                            }
                        }
                        let _ = tx.send_ok(()).await;
                    }
                    Request::Services(tx) => {
                        let m = Response::Services(self.app.schemas());
                        let _ = tx.send_ok(m).await;
                    }
                    Request::Shutdown => {
                        log::info!("supervisor, receive shutdown request.");
                        {
                            let graphs = Arc::clone(&self.graphs);
                            let mut graphs = graphs.lock().await;
                            for (k, v) in graphs.iter() {
                                v.tx.send(()).unwrap();
                                log::info!("stop graph: {:?}", k);
                            }
                            graphs.clear();
                        }
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
