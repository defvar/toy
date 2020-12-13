use crate::api::{graphs, services, tasks};
use crate::auth::{auth_filter, Auth};
use crate::store::{StoreConnection, StoreOpsFactory};
use core::marker::PhantomData;
use std::net::SocketAddr;
use toy_core::error::ServiceError;
use toy_core::mpsc::{Incoming, Outgoing};
use toy_core::supervisor::{Request, SystemMessage};
use warp::http::Method;
use warp::{Filter, Reply};

/// toy-api Server.
#[derive(Debug)]
pub struct Server<C, SF, A> {
    sf: SF,
    auth: A,
    _t: PhantomData<C>,
}

impl<C, SF, A> Server<C, SF, A>
where
    C: StoreConnection + 'static,
    SF: StoreOpsFactory<C> + Clone + 'static,
    A: Auth + Clone + 'static,
{
    pub fn new(sf: SF, auth: A) -> Server<C, SF, A> {
        Server {
            sf,
            auth,
            _t: PhantomData,
        }
    }

    /// Run this `Server` forever on the current thread, specified routes.
    pub async fn run_with_routes<F>(
        &self,
        addr: impl Into<SocketAddr> + 'static,
        mut rx: Incoming<SystemMessage, ServiceError>,
        routes: F,
    ) where
        F: Filter + Clone + Send + Sync + 'static,
        F::Extract: Reply,
    {
        let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(addr, async move {
            while let Some(r) = rx.next().await {
                match r {
                    Ok(r) => match r {
                        SystemMessage::Shutdown => {
                            tracing::info!("shutdown api server because stoped supervisor.");
                            break;
                        }
                    },
                    Err(e) => tracing::error!("error receive system message. error:{:?}", e),
                }
            }
        });
        tracing::info!("listening on http://{}", addr);
        server.await
    }

    /// Run this `Server` forever on the current thread, default routes.
    pub async fn run(
        &self,
        addr: impl Into<SocketAddr> + 'static,
        tx: Outgoing<Request, ServiceError>,
        rx: Incoming<SystemMessage, ServiceError>,
    ) {
        let store_factory = self.sf.clone();
        let store_connection = match store_factory.connect() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("error store connection failed. error:{:?}", e);
                return;
            }
        };
        let routes = auth_filter(self.auth.clone())
            .and(
                graphs(store_connection.clone(), store_factory.clone(), tx.clone())
                    .or(services(tx.clone()))
                    .or(tasks(
                        store_connection.clone(),
                        store_factory.clone(),
                        tx.clone(),
                    )),
            )
            .map(|_, r| r)
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_headers(vec!["authorization"])
                    .allow_methods(&[
                        Method::GET,
                        Method::OPTIONS,
                        Method::POST,
                        Method::DELETE,
                        Method::PUT,
                    ]),
            )
            .with(warp::trace::request());

        self.run_with_routes(addr, rx, routes).await
    }
}
