use crate::context::SupervisorContext;
use crate::exporters::Exporter;
use crate::SupervisorError;
use toy_api::common::PostOption;
use toy_api::task::TaskEvent;
use toy_api_client::client::TaskClient;
use toy_api_client::ApiClient;
use toy_core::metrics::EventRecord;

pub struct ToyExporter<T> {
    client: T,
}

impl<T> ToyExporter<T>
where
    T: ApiClient + Clone,
{
    pub fn new(client: &T) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

#[async_trait::async_trait]
impl<T> Exporter for ToyExporter<T>
where
    T: ApiClient + Clone,
{
    async fn export<C>(
        &self,
        ctx: &SupervisorContext<C>,
        events: Vec<EventRecord>,
    ) -> Result<(), SupervisorError>
    where
        C: ApiClient + Clone + Send + Sync + 'static,
    {
        let mut body = vec![];
        for item in &events {
            body.push(TaskEvent::new(
                item.id(),
                item.name(),
                item.service_type().clone(),
                item.uri().clone(),
                item.event().as_event_text(),
                ctx.name(),
                item.timestamp().clone(),
            ));
        }
        if body.is_empty() {
            return Ok(());
        }

        self.client
            .task()
            .post_event(body, PostOption::new())
            .await
            .map(|_| ())
            .map_err(|x| x.into())
    }
}
