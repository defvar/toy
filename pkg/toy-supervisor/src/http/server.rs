use crate::http::filter::filters;
use crate::supervisor::SupervisorContext;
use std::net::SocketAddr;
use toy_api_client::ApiClient;
use toy_api_http_common::warp;

pub struct Server<C> {
    ctx: SupervisorContext<C>,
}

impl<C> Server<C>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    pub fn new(ctx: SupervisorContext<C>) -> Server<C> {
        Server { ctx }
    }

    /// Run this `Server` forever on the current thread.
    pub async fn run(self, addr: impl Into<SocketAddr> + 'static) {
        let (addr, server) = warp::serve(filters(self.ctx)).bind_ephemeral(addr);
        tracing::info!("listening on http://{}", addr);
        server.await
    }
}
