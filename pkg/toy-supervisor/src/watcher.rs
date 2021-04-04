use crate::{Request, RunTaskResponse};
use futures_util::stream::{self, TryStreamExt};
use toy_api::task::{AllocateOption, AllocateRequest, PendingEntity, WatchOption};
use toy_api_client::client::TaskClient;
use toy_api_client::ApiClient;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;

pub async fn watch<C>(
    name: String,
    c: C,
    tx: Outgoing<Request, ServiceError>,
) -> Result<(), ServiceError>
where
    C: ApiClient + Clone,
{
    loop {
        match c.task().watch(WatchOption::new()).await {
            Ok(st) => {
                let _ = st
                    .map_err(|e| ServiceError::error(e))
                    .map_ok(|x| (x, name.clone(), c.clone(), tx.clone()))
                    .try_for_each(|(x, name, c, tx)| async move {
                        stream::iter(
                            x.pendings()
                                .iter()
                                .map(|x| Ok((x, name.clone(), c.clone(), tx.clone()))),
                        )
                        .try_for_each(
                            |(x, name, c, tx)| async move { request(name, c, x, tx).await },
                        )
                        .await
                    })
                    .await;
            }
            Err(e) => {
                tracing::error!(err = ?e, "an error occured; supervisor when watch task. rerun watcher.");
            }
        }
    }
}

async fn request<C>(
    name: String,
    c: C,
    pending: &PendingEntity,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<(), ServiceError>
where
    C: ApiClient + Clone,
{
    tracing::debug!("watch result :{:?}", pending);
    match pending.graph() {
        Some(graph) => {
            let r = c
                .task()
                .allocate(
                    pending.task_id().to_string(),
                    AllocateRequest::new(name),
                    AllocateOption::new(),
                )
                .await
                .map_err(|e| ServiceError::error(e))?;

            if r.is_ok() {
                let v = toy_core::data::pack(graph).map_err(|e| ServiceError::error(e))?;
                let g = Graph::from(v)?;
                tracing::debug!("{:?}", g);
                let (o_tx, _) = toy_core::oneshot::channel::<RunTaskResponse, ServiceError>();
                let req = Request::RunTask(pending.task_id(), g, o_tx);
                tx.send_ok(req).await
            } else {
                tracing::info!("not found task...running by another supervisor...?");
                Ok(())
            }
        }
        None => Ok(()),
    }
}