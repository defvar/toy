use crate::output::Output;
use crate::{Error, FindCommand};
use std::io::Write;
use toy::api_client::client::{ServiceClient, SupervisorClient};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::FindOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: FindCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let FindCommand {
        resource,
        name,
        pretty,
    } = c;

    let opt = FindOption::new();
    let pretty = pretty.is_some() && pretty.unwrap();

    match resource.as_str() {
        "supervisors" => client
            .supervisor()
            .find(name, opt)
            .await?
            .write(writer, pretty),
        "services" => client
            .service()
            .find(name, opt)
            .await?
            .write(writer, pretty),
        _ => return Err(Error::unknwon_resource(resource)),
    }
}
