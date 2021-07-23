use crate::error::Error;
use crate::opts::ListCommand;
use crate::output::Output;
use std::io::Write;
use toy::api_client::client::{Rbaclient, RoleClient, ServiceClient};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::ListOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: ListCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let ListCommand {
        resource,
        name,
        pretty,
    } = c;

    let opt = ListOption::new();
    let pretty = pretty.is_some() && pretty.unwrap();

    match resource.as_str() {
        "services" => client.service().list(opt).await?.write(writer, pretty),
        "roles" => client.rbac().role().list(opt).await?.write(writer, pretty),
        _ => return Err(Error::unknwon_resource(resource)),
    }
}
