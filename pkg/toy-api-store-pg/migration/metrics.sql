create table metrics
(
    created_at  timestamptz      not null,
    actor       varchar(512)     not null,
    measurement varchar(512)     not null,
    field       varchar(512)     not null,
    counter     bigint           not null default (0),
    gauge       double precision not null default (0)
) with (
    tsdb.hypertable,
    tsdb.partition_column='created_at',
    tsdb.chunk_interval='1 month',
    tsdb.segmentby = 'actor',
    tsdb.orderby = 'created_at DESC'
)
;

create index ix1_metrics on metrics (actor, measurement, created_at desc);
create index ix2_metrics on metrics (measurement, created_at desc);
create index ix3_metrics on metrics (field, created_at desc);
