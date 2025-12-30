use toy_api_client::client::ActorClient;
use toy_api_client::ApiClient;

pub async fn beat<C>(client: C, name: &str, interval: u64)
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!(?name, "beat...");
        if let Err(e) = client.actor().beat(name).await {
            tracing::error!(?name, err= %e, "an error occured; actor when heart beat.");
        }

        toy_rt::sleep(interval).await;
    }
}
