use crate::supervisor::SupervisorContext;
use toy_api_client::ApiClient;

pub async fn event_export<C>(ctx: &SupervisorContext<C>, interval: u64)
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!("event export...");
        let vec = ctx.events().drain().await;
        for item in vec {
            tracing::debug!("{:?}", item);
        }
        toy_rt::sleep(interval).await;
    }
}
