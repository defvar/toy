use crate::{Request, RunTaskResponse};
use futures_util::stream::{self, TryStreamExt};
use toy_api::task::{PendingEntity, WatchOption};
use toy_api_client::client::TaskClient;
use toy_api_client::ApiClient;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;

pub async fn watch<C>(c: C, tx: Outgoing<Request, ServiceError>) -> Result<(), ServiceError>
where
    C: ApiClient,
{
    match c.task().watch(WatchOption::new()).await {
        Ok(st) => {
            st.map_err(|e| ServiceError::error(e))
                .map_ok(|x| (x, tx.clone()))
                .try_for_each(|(x, tx)| async move {
                    stream::iter(x.pendings().iter().map(|x| Ok((x, tx.clone()))))
                        .try_for_each(|(x, tx)| async move { request(x, tx.clone()).await })
                        .await
                })
                .await
        }
        Err(e) => {
            tracing::error!(err = ?e, "an error occured; supervisor when watch task.");
            Err(ServiceError::error(e))
        }
    }
}

async fn request(
    pending: &PendingEntity,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<(), ServiceError> {
    tracing::debug!("{:?}", pending);
    match pending.graph() {
        Some(graph) => {
            let v = toy_core::data::pack(graph).map_err(|e| ServiceError::error(e))?;
            let g = Graph::from(v)?;
            tracing::debug!("{:?}", g);
            let (o_tx, _) = toy_core::oneshot::channel::<RunTaskResponse, ServiceError>();
            let req = Request::RunTask(g, o_tx);
            tx.send_ok(req).await
        }
        None => Ok(()),
    }
}
