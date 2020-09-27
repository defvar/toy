use crate::api::{graphs, services};
use crate::store::{StoreConnection, StoreOpsFactory};
use core::marker::PhantomData;
use std::net::SocketAddr;
use toy_core::error::ServiceError;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::supervisor::{Request, SystemMessage};
use warp::Filter;

/// toy-api Server.
pub struct Server<C, SF> {
    sf: SF,
    _t: PhantomData<C>,
}

impl<C, SF> Server<C, SF>
where
    C: StoreConnection + 'static,
    SF: StoreOpsFactory<C> + 'static + Clone,
{
    pub fn new(sf: SF) -> Server<C, SF> {
        Server {
            sf,
            _t: PhantomData,
        }
    }

    /// Run this `Server` forever on the current thread.
    pub async fn run(
        &self,
        addr: impl Into<SocketAddr> + 'static,
        tx: Outgoing<Request, ServiceError>,
        mut rx: Incoming<SystemMessage, ServiceError>,
    ) {
        let store_factory = self.sf.clone();
        let store_connection = match store_factory.connect() {
            Ok(c) => c,
            Err(e) => {
                log::error!("error store connection failed. error:{:?}", e);
                return;
            }
        };

        let api = graphs(store_connection.clone(), store_factory, tx.clone())
            .or(services(tx.clone()))
            .with(warp::cors().allow_any_origin());

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
