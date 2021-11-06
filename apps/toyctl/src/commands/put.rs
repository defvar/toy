use super::from_file;
use crate::error::Error;
use crate::opts::PutCommand;
use std::io::Write;
use toy::api_client::client::{GraphClient, Rbaclient, RoleBindingClient, RoleClient};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::PutOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: PutCommand, client: HttpApiClient, _writer: W) -> Result<(), Error>
where
    W: Write,
{
    let PutCommand {
        resource,
        name,
        file,
    } = c;

    match resource.as_str() {
        "roles" => {
            client
                .rbac()
                .role()
                .put(name, from_file(file)?, PutOption::new())
                .await?
        }
        "roleBindings" => {
            client
                .rbac()
                .role_binding()
                .put(name, from_file(file)?, PutOption::new())
                .await?
        }
        "graphs" => {
            client
                .graph()
                .put(name, from_file(file)?, PutOption::new())
                .await?
        }
        _ => return Err(Error::unknwon_resource(resource)),
    };
    Ok(())
}
