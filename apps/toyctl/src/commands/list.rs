use crate::error::Error;
use crate::opts::ListCommand;
use crate::output::Output;
use std::io::Write;
use toy::api_client::client::{
    Rbaclient, RoleBindingClient, RoleClient, ServiceClient, SupervisorClient, TaskClient,
};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::ListOption;
use toy::api_client::toy_api::services::ServiceSpecListOption;
use toy::api_client::toy_api::supervisors::SupervisorListOption;
use toy::api_client::toy_api::task::TaskListOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: ListCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let ListCommand { resource, pretty } = c;

    let opt = ListOption::new();
    let pretty = pretty.is_some() && pretty.unwrap();

    match resource.as_str() {
        "supervisors" => client
            .supervisor()
            .list(SupervisorListOption::new())
            .await?
            .write(writer, pretty),
        "services" => client
            .service()
            .list(ServiceSpecListOption::new())
            .await?
            .write(writer, pretty),
        "roles" => client.rbac().role().list(opt).await?.write(writer, pretty),
        "roleBindings" => client
            .rbac()
            .role_binding()
            .list(opt)
            .await?
            .write(writer, pretty),
        "tasks" => client
            .task()
            .list(TaskListOption::new())
            .await?
            .write(writer, pretty),
        _ => return Err(Error::unknwon_resource(resource)),
    }
}
