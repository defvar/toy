use super::from_file;
use crate::error::Error;
use crate::opts::{PostCommand, PostResources};
use crate::output::Output;
use std::io::Write;
use toy::api_client::client::TaskClient;
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::common::PostOption;
use toy::api_client::ApiClient;

pub async fn execute<W>(c: PostCommand, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    let PostCommand { resource } = c;

    match resource {
        PostResources::Tasks(c) => client
            .task()
            .post(from_file(c.file)?, PostOption::new())
            .await
            .write(writer, false),
    }
}
