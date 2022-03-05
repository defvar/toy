use super::from_file;
use crate::error::Error;
use crate::opts::PostCommand;
use crate::output::Output;
use std::io::Write;
use toy::api_client::client::TaskClient;
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::task::PostOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: PostCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let PostCommand { resource, file } = c;

    match resource.as_str() {
        "tasks" => client
            .task()
            .post(from_file(file)?, PostOption::new())
            .await
            .write(writer, false),
        _ => return Err(Error::unknwon_resource(resource)),
    }
}
