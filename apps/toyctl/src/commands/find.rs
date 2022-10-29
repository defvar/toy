use crate::output::Output;
use crate::{Error, FindCommand, FindResources};
use std::io::Write;
use toy::api_client::client::{GraphClient, ServiceClient, SupervisorClient};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::FindOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: FindCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let FindCommand { resource, pretty } = c;

    let opt = FindOption::new();

    match resource {
        FindResources::Supervisors(c) => client
            .supervisor()
            .find(c.name, opt)
            .await
            .write(writer, pretty),
        FindResources::Services(c) => client
            .service()
            .find(c.name, opt)
            .await
            .write(writer, pretty),
        FindResources::Graphs(c) => client.graph().find(c.name, opt).await.write(writer, pretty),
    }
}
