use crate::api::{graphs, services, tasks};
use crate::auth::{auth_filter, Auth};
use crate::config::ServerConfig;
use crate::graph::store::GraphStoreOps;
use crate::store::{StoreConnection, StoreOpsFactory};
use crate::task::store::TaskLogStore;
use core::marker::PhantomData;
use std::net::SocketAddr;
use toy::core::error::ServiceError;
use toy::core::mpsc::{Incoming, Outgoing};
use toy::supervisor::{Request, SystemMessage};
use warp::http::Method;
use warp::{Filter, Reply};

/// toy-api Server.
#[derive(Debug)]
pub struct Server<C, SF, A, Config> {
    sf: SF,
    auth: A,
    client: Option<reqwest::Client>,
    config: Config,
    _t: PhantomData<C>,
}

impl<C, S, SF, A, Config> Server<C, SF, A, Config>
where
    C: StoreConnection + 'static,
    S: GraphStoreOps<C> + 'static,
    SF: StoreOpsFactory<Con = C, Ops = S> + Clone + 'static,
    A: Auth + Clone + 'static,
    Config: ServerConfig,
{
    pub fn new(sf: SF, auth: A, config: Config) -> Server<C, SF, A, Config> {
        Server {
            sf,
            auth,
            client: None,
            config,
            _t: PhantomData,
        }
    }

    /// Use http client.
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
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
        let store_ops = self.sf.create().unwrap();
        let store_connection = match self.sf.connect() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("error store connection failed. error:{:?}", e);
                return;
            }
        };

        let client = match self.client {
            Some(ref c) => c.clone(),
            None => match reqwest::Client::builder().build() {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("error client builder failed. error:{:?}", e);
                    return;
                }
            },
        };
        let mut log_store = self.config.task_log_store();
        if let Err(e) = log_store.establish(client.clone()) {
            tracing::error!("error log store connection failed. error:{:?}", e);
            return;
        };
        let routes = auth_filter(self.config.auth(), client)
            .and(
                graphs(store_connection.clone(), store_ops.clone(), tx.clone())
                    .or(services(tx.clone()))
                    .or(tasks(log_store, tx.clone())),
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
