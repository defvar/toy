create table task_events
(
    created_at timestamptz    not null,
    supervisor varchar(512)   not null,
    event      jsonb          not null
) with (
    tsdb.hypertable,
    tsdb.partition_column='created_at',
    tsdb.chunk_interval='1 month',
    tsdb.segmentby = 'supervisor',
    tsdb.orderby = 'created_at DESC'
)
;

create index ix1_task_events on task_events (supervisor, created_at desc);
create index ix2_task_events on task_events using gin (event);
