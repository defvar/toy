use crate::metrics::context::metrics;
use serde::Serialize;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MeasureKind {
    Counter,
    Gauge,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MetricsKind {
    StartTask,
    FinishTask,
    StartService,
    FinishService,
    SendRequest,
    ReceiveRequest,
    ReceiveError,
    ReceiveStop,

    RunningTask,
    RunningService,

    Custom(String, MeasureKind),
}

impl MetricsKind {
    pub fn as_kind_text(&self) -> &str {
        match self {
            MetricsKind::StartTask => "start_task",
            MetricsKind::FinishTask => "finish_task",
            MetricsKind::StartService => "start_service",
            MetricsKind::FinishService => "finish_service",
            MetricsKind::SendRequest => "send_request",
            MetricsKind::ReceiveRequest => "receive_request",
            MetricsKind::ReceiveError => "receive_error",
            MetricsKind::ReceiveStop => "receive_stop",

            MetricsKind::RunningTask => "running_task",
            MetricsKind::RunningService => "running_service",

            MetricsKind::Custom(v, _) => v,
        }
    }

    pub fn as_measure_kind(&self) -> MeasureKind {
        match self {
            MetricsKind::StartTask => MeasureKind::Counter,
            MetricsKind::FinishTask => MeasureKind::Counter,
            MetricsKind::StartService => MeasureKind::Counter,
            MetricsKind::FinishService => MeasureKind::Counter,
            MetricsKind::SendRequest => MeasureKind::Counter,
            MetricsKind::ReceiveRequest => MeasureKind::Counter,
            MetricsKind::ReceiveError => MeasureKind::Counter,
            MetricsKind::ReceiveStop => MeasureKind::Counter,

            MetricsKind::RunningTask => MeasureKind::Gauge,
            MetricsKind::RunningService => MeasureKind::Gauge,

            MetricsKind::Custom(_, v) => *v,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum MetricsEventKind {
    StartTask,
    FinishTask,
    StartService,
    FinishService,
    SendRequest,
    ReceiveRequest,
    ReceiveError,
    ReceiveStop,
    ReceiveUpstreamFinish,
    FinishUpstreamFinish,
    FinishUpstreamFinishAll,
    CustomEvent(String),
}

impl MetricsEventKind {
    pub fn as_event_text(&self) -> &str {
        match self {
            MetricsEventKind::StartTask => "start_task",
            MetricsEventKind::FinishTask => "finish_task",
            MetricsEventKind::StartService => "start_service",
            MetricsEventKind::FinishService => "finish_service",
            MetricsEventKind::SendRequest => "send_request",
            MetricsEventKind::ReceiveRequest => "receive_request",
            MetricsEventKind::ReceiveError => "receive_error",
            MetricsEventKind::ReceiveStop => "receive_stop",
            MetricsEventKind::ReceiveUpstreamFinish => "receive_upstream_finish",
            MetricsEventKind::FinishUpstreamFinish => "finish_upstream_finish",
            MetricsEventKind::FinishUpstreamFinishAll => "finish_upstream_finish_all",
            MetricsEventKind::CustomEvent(e) => e,
        }
    }

    pub(crate) async fn apply_metrics(&self) {
        match self {
            MetricsEventKind::StartTask => {
                inc_counter(&MetricsKind::StartTask).await;
                inc_gauge(&MetricsKind::RunningTask).await;
            }
            MetricsEventKind::FinishTask => {
                inc_counter(&MetricsKind::FinishTask).await;
                dec_gauge(&MetricsKind::RunningTask).await;
            }
            MetricsEventKind::StartService => {
                inc_counter(&MetricsKind::StartService).await;
                inc_gauge(&MetricsKind::RunningService).await;
            }
            MetricsEventKind::FinishService => {
                inc_counter(&MetricsKind::FinishService).await;
                dec_gauge(&MetricsKind::RunningService).await;
            }
            MetricsEventKind::SendRequest => {
                inc_counter(&MetricsKind::SendRequest).await;
            }
            MetricsEventKind::ReceiveRequest => {
                inc_counter(&MetricsKind::ReceiveRequest).await;
            }
            MetricsEventKind::ReceiveError => {
                inc_counter(&MetricsKind::ReceiveError).await;
            }
            MetricsEventKind::ReceiveStop => {
                inc_counter(&MetricsKind::ReceiveStop).await;
            }
            _ => (),
        }
    }
}

async fn inc_counter(k: &MetricsKind) {
    metrics().counter(k, |c| c.increment()).await;
}

async fn inc_gauge(k: &MetricsKind) {
    metrics().gauge(k, |c| c.increment(1.0)).await;
}

async fn dec_gauge(k: &MetricsKind) {
    metrics().gauge(k, |c| c.decrement(1.0)).await;
}
