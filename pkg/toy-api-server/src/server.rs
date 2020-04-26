use crate::GraphRegistry;
use crate::{graphs, services};
use std::net::SocketAddr;
use toy_core::error::ServiceError;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::supervisor::{Request, SystemMessage};
use warp::Filter;

pub struct Server {
    graphs: GraphRegistry,
}

impl Server {
    pub fn new(graphs: GraphRegistry) -> Server {
        Server { graphs }
    }

    pub async fn run(
        self,
        addr: impl Into<SocketAddr> + 'static,
        tx: Outgoing<Request, ServiceError>,
        mut rx: Incoming<SystemMessage, ServiceError>,
    ) {
        let api = graphs(self.graphs, tx.clone()).or(services(tx.clone()));

        let (addr, server) = warp::serve(api).bind_with_graceful_shutdown(addr, async move {
            while let Some(r) = rx.next().await {
                match r {
                    Ok(r) => match r {
                        SystemMessage::Shutdown => {
                            log::info!("shutdown api server because stoped supervisor.");
                            break;
                        }
                    },
                    Err(e) => log::error!("error receive system message. error:{:?}", e),
                }
            }
        });
        log::info!("listening on http://{}", addr);
        server.await
    }
}
