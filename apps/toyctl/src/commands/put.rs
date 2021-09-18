use crate::error::Error;
use crate::opts::PutCommand;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use toy::api_client::client::{Rbaclient, RoleBindingClient, RoleClient};
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
        _ => return Err(Error::unknwon_resource(resource)),
    };
    Ok(())
}

fn from_file<T>(file: PathBuf) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut f = File::open(file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let v = toy_pack_json::unpack(&buffer)?;
    Ok(v)
}
