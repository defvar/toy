use super::from_file;
use crate::error::Error;
use crate::opts::{PutCommand, PutResources};
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
    let PutCommand { resource, pretty } = c;

    match resource {
        PutResources::Roles(c) => client
            .rbac()
            .role()
            .put(c.name, from_file(c.file)?, PutOption::new())
            .await
            .write(writer, pretty),
        PutResources::RoleBindings(c) => client
            .rbac()
            .role_binding()
            .put(c.name, from_file(c.file)?, PutOption::new())
            .await
            .write(writer, pretty),
        PutResources::Graphs(c) => client
            .graph()
            .put(c.name, from_file(c.file)?, PutOption::new())
            .await
            .write(writer, pretty),
    }
}
