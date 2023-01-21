use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use toy_api::metrics::{Metrics, MetricsEntry};
use toy_api::selection::selector::Predicate;
use toy_api::selection::Operator;
use toy_api::task::{TaskEvent, TaskEventList, TaskList};
use toy_api_server::store::error::StoreError;
use toy_api_server::store::metrics::{MetricsStore, MetricsStoreOps};
use toy_api_server::store::task_event::{
    CreateOption, ListEventOption, ListTaskOption, TaskEventStore, TaskEventStoreOps,
};
use toy_api_server::store::StoreConnection;
use toy_api_server::toy_h::HttpClient;
use toy_core::data::Value;
use toy_influxdb::models::line_protocol::LineProtocolBuilder;
use toy_influxdb::models::query_param::QueryParam;
use toy_influxdb::models::FieldValue;
use toy_influxdb::query::builder::{Filter, FluxBuilder};
use toy_influxdb::Client;

#[derive(Clone, Debug)]
pub struct InfluxdbStore<T> {
    con: Option<InfluxdbConnection<T>>,
}

#[derive(Clone)]
pub struct InfluxdbConnection<T> {
    client: Client<T>,
    token: String,
    org: String,
    bucket: String,
}

#[derive(Clone, Debug)]
pub struct InfluxdbStoreOps<T> {
    _t: PhantomData<T>,
}

impl<T> InfluxdbStore<T> {
    pub fn new() -> Self {
        InfluxdbStore { con: None }
    }
}

impl<T> Debug for InfluxdbConnection<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InfluxdbConnection")
            .field("client", &self.client)
            .finish()
    }
}

fn env(key: &str) -> Result<String, StoreError> {
    if let Ok(v) = std::env::var(key) {
        Ok(v)
    } else {
        Err(StoreError::error(format!("must be set [{}].", key)))
    }
}

impl<T> InfluxdbStore<T>
where
    T: HttpClient,
{
    fn establish_influxdb(
        &mut self,
        client: T,
        store_name: &'static str,
    ) -> Result<(), StoreError> {
        if self.con.is_some() {
            return Ok(());
        }

        let url = std::env::var("TOY_API_TASK_EVENT_STORE_INFLUXDB_API_URL")
            .unwrap_or_else(|_| "http://localhost:8086".to_string());
        let token = env("TOY_API_TASK_EVENT_STORE_INFLUXDB_API_TOKEN")?;
        let org = env("TOY_API_TASK_EVENT_STORE_INFLUXDB_ORG")?;
        let bucket = env("TOY_API_TASK_EVENT_STORE_INFLUXDB_BUCKET")?;
        tracing::info!("toy {} store=influxdb. connecting:{}", store_name, &url);
        match Client::from(client, url) {
            Ok(c) => {
                self.con = Some(InfluxdbConnection {
                    client: c,
                    token,
                    org,
                    bucket,
                });
            }
            Err(e) => return Err(StoreError::error(e)),
        };
        Ok(())
    }
}

impl<T> StoreConnection for InfluxdbConnection<T> where T: HttpClient {}

impl<T> TaskEventStore<T> for InfluxdbStore<T>
where
    T: HttpClient,
{
    type Con = InfluxdbConnection<T>;
    type Ops = InfluxdbStoreOps<T>;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        InfluxdbStoreOps { _t: PhantomData }
    }

    fn establish(&mut self, client: T) -> Result<(), StoreError> {
        self.establish_influxdb(client, "task event store")
    }
}

impl<T> MetricsStore<T> for InfluxdbStore<T>
where
    T: HttpClient,
{
    type Con = InfluxdbConnection<T>;
    type Ops = InfluxdbStoreOps<T>;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        InfluxdbStoreOps { _t: PhantomData }
    }

    fn establish(&mut self, client: T) -> Result<(), StoreError> {
        self.establish_influxdb(client, "metrics store")
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> TaskEventStoreOps for InfluxdbStoreOps<T>
where
    T: HttpClient,
{
    type Con = InfluxdbConnection<T>;
    type Err = StoreError;

    async fn list_event(
        &self,
        con: Self::Con,
        opt: ListEventOption,
    ) -> Result<TaskEventList, Self::Err> {
        let name_filter = to_name_filter(opt.name());
        let query = FluxBuilder::from("toy")
            .range(opt.start().copied(), opt.stop().copied())
            .filter(Filter::eq("_measurement", "event".into()))
            .filter(name_filter)
            .ungroup()
            .pivot(
                &["_time", "name", "supervisor", "service_type", "uri"],
                &["_field"],
                "_value",
            )
            .sort(&["_time"])
            .rename(&[("_time", "timestamp")])
            .limit(opt.limit().unwrap_or_else(|| 0))
            .to_flux()
            .map_err(|e| StoreError::error(e))?;

        let param = QueryParam::with(query);

        let result = con
            .client
            .query(&con.token, &con.org, &param)
            .await
            .map_err(|e| StoreError::error(e))?
            .unpack()
            .map_err(|e| StoreError::error(e))?;

        Ok(TaskEventList::new(result))
    }

    async fn list_task(
        &self,
        con: Self::Con,
        opt: ListTaskOption,
    ) -> Result<toy_api::task::TaskList, Self::Err> {
        let name_filter = to_name_filter(opt.name());
        let query = FluxBuilder::from("toy")
            .range(opt.start().copied(), opt.stop().copied())
            .filter(Filter::eq("_measurement", "event".into()))
            .filter(name_filter)
            .ungroup()
            .pivot(
                &["_time", "name", "supervisor", "service_type", "uri"],
                &["_field"],
                "_value",
            )
            .filter(Filter::regex_match("event", "Task".into()))
            .drop(&["uri", "service_type"])
            .sort(&["task_id", "_time"])
            .rename(&[("_time", "timestamp")])
            .limit(opt.limit().unwrap_or_else(|| 0))
            .to_flux()
            .map_err(|e| StoreError::error(e))?;

        let param = QueryParam::with(query);
        let result = con
            .client
            .query(&con.token, &con.org, &param)
            .await
            .map_err(|e| StoreError::error(e))?
            .unpack()
            .map_err(|e| StoreError::error(e))?;

        Ok(TaskList::new(result))
    }

    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        _opt: CreateOption,
    ) -> Result<(), Self::Err> {
        let mut builder = LineProtocolBuilder::new();
        for e in &events {
            builder
                .start_record("event", e.timestamp().clone())
                .tag("name", e.name())
                .tag("supervisor", e.supervisor())
                .tag("service_type", e.service_type().full_name())
                .tag("uri", e.uri().as_ref())
                .field("event_id", FieldValue::String(e.event_id().to_string()))
                .field("task_id", FieldValue::String(e.task_id().to_string()))
                .field("event", FieldValue::String(e.event().to_string()))
                .end_record();
        }
        let records = builder.build();
        con.client
            .write(&con.token, &con.bucket, &con.org, records)
            .await
            .map_err(|e| StoreError::error(e))?;
        Ok(())
    }
}

fn to_field_value(value: &Value) -> FieldValue {
    match value {
        Value::Bool(v) => FieldValue::Boolean(*v),
        Value::Integer(v) => FieldValue::Integer(*v),
        Value::Number(v) => FieldValue::Float(*v),
        Value::String(v) => FieldValue::String(v.clone()),
        Value::TimeStamp(v) => FieldValue::Timestamp(v.clone()),
        Value::None => FieldValue::Nil,
        _ => FieldValue::Nil,
    }
}

fn to_name_filter(predicate: Option<&Predicate>) -> Filter {
    if let Some(name) = predicate {
        let v = to_field_value(name.value());
        match name.op() {
            Operator::Eq => Filter::eq(name.field(), v),
            Operator::NotEq => Filter::ne(name.field(), v),
            Operator::LessThan => Filter::less_than(name.field(), v),
            Operator::LessThanOrEqual => Filter::less_than_or_equal(name.field(), v),
            Operator::GreaterThan => Filter::greater_than(name.field(), v),
            Operator::GreaterThanOrEqual => Filter::greater_than_or_equal(name.field(), v),
            Operator::Match => Filter::regex_match(name.field(), v),
            Operator::Unmatch => Filter::regex_not_match(name.field(), v),
        }
    } else {
        Filter::none()
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> MetricsStoreOps for InfluxdbStoreOps<T>
where
    T: HttpClient,
{
    type Con = InfluxdbConnection<T>;
    type Err = StoreError;

    async fn create(
        &self,
        con: Self::Con,
        metrics: Metrics,
        _opt: toy_api_server::store::metrics::CreateOption,
    ) -> Result<(), Self::Err> {
        let mut builder = LineProtocolBuilder::new();
        builder
            .start_record(metrics.measurement(), metrics.timestamp().clone())
            .tag("supervisor", metrics.supervisor());

        for tag in metrics.tags() {
            builder.tag(tag.key(), tag.value());
        }

        for item in metrics.items() {
            match item {
                MetricsEntry::Counter(v) => {
                    builder.field(v.name(), FieldValue::UInteger(v.value()))
                }
                MetricsEntry::Gauge(v) => builder.field(v.name(), FieldValue::Float(v.value())),
            };
        }
        builder.end_record();
        let records = builder.build();
        con.client
            .write(&con.token, &con.bucket, &con.org, records)
            .await
            .map_err(|e| StoreError::error(e))?;
        Ok(())
    }
}
