use super::from_file;
use crate::error::Error;
use crate::opts::PutCommand;
use crate::output::Output;
use std::io::Write;
use toy::api_client::client::{GraphClient, Rbaclient, RoleBindingClient, RoleClient};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::PutOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: PutCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let PutCommand {
        resource,
        name,
        file,
        pretty,
    } = c;

    let pretty = pretty.is_some() && pretty.unwrap();

    match resource.as_str() {
        "roles" => client
            .rbac()
            .role()
            .put(name, from_file(file)?, PutOption::new())
            .await
            .write(writer, pretty),
        "roleBindings" => client
            .rbac()
            .role_binding()
            .put(name, from_file(file)?, PutOption::new())
            .await
            .write(writer, pretty),
        "graphs" => client
            .graph()
            .put(name, from_file(file)?, PutOption::new())
            .await
            .write(writer, pretty),
        _ => return Err(Error::unknwon_resource(resource)),
    }
}
