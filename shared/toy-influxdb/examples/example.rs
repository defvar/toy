use chrono::Utc;
use toy_h::impl_reqwest::ReqwestClient;
use toy_influxdb::models::line_protocol::LineProtocolBuilder;
use toy_influxdb::models::query_param::QueryParam;
use toy_influxdb::models::FieldValue;
use toy_influxdb::{Client, InfluxDBError};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), InfluxDBError> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let client = ReqwestClient::from(toy_h::reqwest::Client::builder().build().unwrap());
    let client = Client::from(client, "http://localhost:8086").unwrap();
    let token = std::env::var("TOY_SHARE_INFLUXDB_EXAMPLE_TOKEN")
        .expect("please set env [TOY_SHARE_INFLUXDB_EXAMPLE_TOKEN]");

    //
    // write
    //
    let mut builder = LineProtocolBuilder::new();
    let tests = test_data();
    for t in &tests {
        builder
            .start_record("toy_example", Utc::now())
            .tag("name", &t.name)
            .tag("category", &t.name)
            .field("va", FieldValue::Integer(100 + t.number))
            .field("va2", FieldValue::Integer(200 + t.number))
            .end_record();
    }
    let records = builder.build();
    let r = client.write(&token, "toy", "toy", records).await;
    println!("{:?}", r);

    //
    // query
    //
    let query = "from(bucket: \"toy\") |> range(start: -2d) |> filter(fn: (r) => r._measurement == \"toy_example\") |> group() |> sort(columns: [\"_time\"])".to_string();
    //let query = "from(bucket: \"toy\") |> range(start: -2d) |> filter(fn: (r) => r._measurement == \"events\") |> group() |> sort(columns: [\"_time\"]) |> keep(columns: [\"_time\", \"_value\", \"name\", \"supervisor_name\", \"uri\"])".to_string();

    let param = QueryParam::with(query);
    let r = client.query(&token, "toy", &param).await;

    println!("{:?}", r);

    Ok(())
}

struct Data {
    number: i64,
    name: String,
}

fn test_data() -> Vec<Data> {
    let mut tests = Vec::new();
    for i in 0..10 {
        tests.push(Data {
            number: i,
            name: format!("name-{}", i),
        });
    }
    tests
}
