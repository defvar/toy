use crate::error::Error;
use crate::opts::{ListCommand, ListResources};
use crate::output::Output;
use serde::de::DeserializeOwned;
use std::io::Write;
use toy::api_client::client::{
    GraphClient, Rbaclient, RoleBindingClient, RoleClient, ServiceClient, SupervisorClient,
    TaskClient,
};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::ListOption;
use toy::api_client::toy_api::services::ServiceSpecListOption;
use toy::api_client::toy_api::supervisors::SupervisorListOption;
use toy::api_client::toy_api::task::TaskListOption;
use toy::api_client::ApiClient;
use toy_pack_urlencoded::QueryParseError;

pub async fn execute<W>(c: ListCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let ListCommand { resource, pretty } = c;

    match resource {
        ListResources::Supervisors(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .supervisor()
                .list(opt.unwrap_or(SupervisorListOption::new()))
                .await
                .write(writer, pretty)
        }
        ListResources::Services(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .service()
                .list(opt.unwrap_or(ServiceSpecListOption::new()))
                .await
                .write(writer, pretty)
        }
        ListResources::Graphs(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .graph()
                .list(opt.unwrap_or(ListOption::new()))
                .await
                .write(writer, pretty)
        }
        ListResources::Roles(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .rbac()
                .role()
                .list(opt.unwrap_or(ListOption::new()))
                .await
                .write(writer, pretty)
        }
        ListResources::RoleBindings(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .rbac()
                .role_binding()
                .list(opt.unwrap_or(ListOption::new()))
                .await
                .write(writer, pretty)
        }
        ListResources::Tasks(c) => {
            let opt = parse_opt(c.opt.as_ref())?;
            client
                .task()
                .list(opt.unwrap_or(TaskListOption::new()))
                .await
                .write(writer, pretty)
        }
    }
}

fn parse_opt<T: DeserializeOwned>(selector: Option<&String>) -> Result<Option<T>, QueryParseError> {
    if selector.is_some() {
        toy_pack_urlencoded::unpack(selector.unwrap().as_bytes()).map(Some)
    } else {
        Ok(None)
    }
}
