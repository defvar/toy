use toy_api_client::client::SupervisorClient;
use toy_api_client::ApiClient;

pub async fn beat<C>(client: C, name: String, interval: u64)
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    loop {
        tracing::debug!(?name, "beat...");
        if let Err(e) = client.supervisor().beat(&name).await {
            tracing::error!(?name, err= %e, "an error occured; supervisor when heart beat.");
        }

        toy_rt::sleep(interval).await;
    }
}
