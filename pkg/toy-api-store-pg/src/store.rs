use chrono::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Json;
use sqlx::types::chrono::Utc;
use sqlx::{FromRow, Pool, Postgres, QueryBuilder};
use std::str::FromStr;
use toy_api::metrics::{Metrics, MetricsEntry};
use toy_api::selection::Operator;
use toy_api::selection::selector::Predicate;
use toy_api::task::{Task, TaskEvent, TaskEventList, TaskList};
use toy_api_server::store::StoreConnection;
use toy_api_server::store::error::StoreError;
use toy_api_server::store::metrics::{MetricsStore, MetricsStoreOps};
use toy_api_server::store::task_event::{
    CreateOption, ListEventOption, ListTaskOption, TaskEventStore, TaskEventStoreOps,
};
use toy_core::metrics::MetricsEventKind;
use tracing::instrument;

static DEFAULT_MAX_CONNECTIONS: u32 = 5;
static DEFAULT_BATCH_SIZE: usize = 1000;

#[derive(Debug, Clone)]
pub struct PgStore {
    con: Option<PgStoreConnection>,
}

#[derive(Debug, Clone)]
pub struct PgStoreConnection {
    raw: Pool<Postgres>,
    batch_size: usize,
}

pub struct PgStoreOps;

impl PgStore {
    pub fn new() -> Self {
        PgStore { con: None }
    }
}

impl StoreConnection for PgStoreConnection {}

fn env(key: &str) -> Result<String, StoreError> {
    if let Ok(v) = std::env::var(key) {
        Ok(v)
    } else {
        Err(StoreError::error(format!("must be set [{}].", key)))
    }
}

fn env_from_str<T: FromStr>(key: &str, default: T) -> Result<T, StoreError> {
    if let Ok(v) = env(key) {
        let candidate = T::from_str(&v);
        if let Ok(v) = candidate {
            Ok(v)
        } else {
            Err(StoreError::error(format!("invalid value [{}].", key)))
        }
    } else {
        Ok(default)
    }
}

impl PgStore {
    fn establish(&mut self, store_name: &'static str) -> Result<(), StoreError> {
        if self.con.is_some() {
            return Ok(());
        }

        let url = env("TOY_API_TASK_EVENT_STORE_POSTGRES_URL")?;
        let max_connections = env_from_str(
            "TOY_API_TASK_EVENT_STORE_POSTGRES_MAX_CONNECTIONS",
            DEFAULT_MAX_CONNECTIONS,
        )?;
        let batch_size = env_from_str(
            "TOY_API_TASK_EVENT_STORE_POSTGRES_DEFAULT_BATCH_SIZE",
            DEFAULT_BATCH_SIZE,
        )?;

        tracing::info!("toy {} store=postgres. connecting:{}", store_name, url);
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect_lazy(&url);

        match pool {
            Ok(c) => self.con = Some(PgStoreConnection { raw: c, batch_size }),
            Err(e) => return Err(StoreError::error(e)),
        };
        Ok(())
    }
}

impl<T> TaskEventStore<T> for PgStore {
    type Con = PgStoreConnection;
    type Ops = PgStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        PgStoreOps
    }

    fn establish(&mut self, _: T) -> Result<(), StoreError> {
        self.establish("task event store")
    }
}

impl<T> MetricsStore<T> for PgStore {
    type Con = PgStoreConnection;
    type Ops = PgStoreOps;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        PgStoreOps
    }

    fn establish(&mut self, _client: T) -> Result<(), StoreError> {
        self.establish("metrics store")
    }
}

#[derive(FromRow)]
struct TaskEventRow {
    event: Json<TaskEvent>,
}

#[derive(FromRow)]
struct TaskRow {
    event: Json<Task>,
}

#[toy_api_server::async_trait::async_trait]
impl TaskEventStoreOps for PgStoreOps {
    type Con = PgStoreConnection;
    type Err = StoreError;

    #[instrument(level = "debug", skip(self, con))]
    async fn list_event(
        &self,
        con: Self::Con,
        opt: ListEventOption,
    ) -> Result<TaskEventList, Self::Err> {
        let now = Utc::now();
        let start = opt.start().cloned().unwrap_or(now - Duration::minutes(10));

        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("select event from task_events where 0=0");

        builder.push(" and created_at >= ");
        builder.push_bind(start);

        builder.push(" and created_at <= ");
        builder.push_bind(opt.stop().cloned().unwrap_or(now));

        if let Some(name) = opt.name() {
            builder.push(format!(" and event['name']::text {} ", predicate_str(name)));
            builder.push_bind(name.value().as_str().unwrap());
        }

        builder.push(" order by created_at desc");
        builder.push(" limit ");
        builder.push_bind(opt.limit().unwrap_or(1000) as i32);

        let query = builder.build_query_as();
        let events: Vec<TaskEventRow> =
            query.fetch_all(&con.raw).await.map_err(StoreError::error)?;
        Ok(TaskEventList::new(
            events.into_iter().map(|x| x.event.0).collect::<Vec<_>>(),
        ))
    }

    #[instrument(level = "debug", skip(self, con))]
    async fn list_task(&self, con: Self::Con, opt: ListTaskOption) -> Result<TaskList, Self::Err> {
        let now = Utc::now();
        let start = opt.start().cloned().unwrap_or(now - Duration::minutes(10));

        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("select event from task_events where 0=0");

        builder.push(" and created_at >= ");
        builder.push_bind(start);

        builder.push(" and created_at <= ");
        builder.push_bind(opt.stop().cloned().unwrap_or(now));

        builder.push(format!(
            " event['event'] = '\"{}\"' ",
            MetricsEventKind::StartTask.as_event_text()
        ));

        if let Some(name) = opt.name() {
            builder.push(format!(" and event['name']::text {} ", predicate_str(name)));
            builder.push_bind(name.value().as_str().unwrap());
        }

        builder.push(" order by created_at desc");
        builder.push(" limit ");
        builder.push_bind(opt.limit().unwrap_or(1000) as i32);

        let query = builder.build_query_as();
        let events: Vec<TaskRow> = query.fetch_all(&con.raw).await.map_err(StoreError::error)?;

        Ok(TaskList::new(
            events.into_iter().map(|x| x.event.0).collect::<Vec<_>>(),
        ))
    }

    #[instrument(level = "debug", skip(self, con, events))]
    async fn create(
        &self,
        con: Self::Con,
        events: Vec<TaskEvent>,
        _opt: CreateOption,
    ) -> Result<(), Self::Err> {
        for chunk in events.chunks(con.batch_size) {
            let mut builder: QueryBuilder<Postgres> =
                QueryBuilder::new("insert into task_events(created_at, actor, event)");

            builder.push_values(chunk, |mut b, event| {
                b.push_bind(event.timestamp())
                    .push_bind(event.actor())
                    .push_bind(Json(event));
            });

            builder
                .build()
                .persistent(false)
                .execute(&con.raw)
                .await
                .map_err(StoreError::error)
                .map(|x| {
                    tracing::debug!("create event:{} rows affected.", x.rows_affected());
                })?;
        }
        Ok(())
    }
}

#[toy_api_server::async_trait::async_trait]
impl MetricsStoreOps for PgStoreOps {
    type Con = PgStoreConnection;
    type Err = StoreError;

    #[instrument(level = "debug", skip(self, con, metrics))]
    async fn create(
        &self,
        con: Self::Con,
        metrics: Metrics,
        _opt: toy_api_server::store::metrics::CreateOption,
    ) -> Result<(), Self::Err> {
        let sv = metrics.actor();
        let measurement = metrics.measurement();
        let time = metrics.timestamp();

        for chunk in metrics.items().chunks(con.batch_size) {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "insert into metrics(created_at, actor, measurement, field, counter, gauge)",
            );

            builder.push_values(chunk, |mut b, metric| {
                let (counter, gauge) = match metric {
                    MetricsEntry::Counter(v) => (v.value(), 0f64),
                    MetricsEntry::Gauge(v) => (0u64, v.value()),
                };
                b.push_bind(time)
                    .push_bind(sv)
                    .push_bind(measurement)
                    .push_bind(metric.name())
                    .push_bind(counter as i64)
                    .push_bind(gauge);
            });

            builder
                .build()
                .persistent(false)
                .execute(&con.raw)
                .await
                .map_err(StoreError::error)
                .map(|x| {
                    tracing::debug!("create event:{} rows affected.", x.rows_affected());
                })?;
        }
        Ok(())
    }
}

fn predicate_str(pred: &Predicate) -> &str {
    match pred.op() {
        Operator::Eq => " = ",
        Operator::NotEq => " != ",
        Operator::GreaterThan => " > ",
        Operator::GreaterThanOrEqual => " >= ",
        Operator::LessThan => " < ",
        Operator::LessThanOrEqual => " <= ",
        Operator::Match => " ~ ",
        Operator::Unmatch => " !~ ",
    }
}
