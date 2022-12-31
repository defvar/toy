use toy_core::data::Value;
use toy_core::graph::Graph;
use toy_core::metrics::context::metrics;
use toy_core::metrics::kind::MetricsKind;
use toy_core::metrics::registry::metrics::MetricsRegistry;
use toy_core::metrics::MetricsEventKind;
use toy_core::task::{TaskContext, TaskId};
use toy_core::{map_value, seq_value, ServiceType, Uri};

#[tokio::test]
async fn metrics_counter() {
    let reg = MetricsRegistry::new();
    reg.counter(&MetricsKind::StartTask, |c| c.increment())
        .await;
    let counters = reg.get_counters().await;
    assert_eq!(counters.len(), 1);
    assert_eq!(
        counters.get(&MetricsKind::StartTask).unwrap().get(),
        Some(1)
    );
}

#[tokio::test]
async fn metrics_gauge() {
    let reg = MetricsRegistry::new();
    reg.gauge(&MetricsKind::StartTask, |c| c.increment(1.1))
        .await;
    let gauges = reg.get_gauges().await;
    assert_eq!(gauges.len(), 1);
    assert_eq!(
        gauges.get(&MetricsKind::StartTask).unwrap().get(),
        Some(1.1)
    );
}

#[tokio::test]
async fn applyy_metrics_start_task() {
    let ctx = TaskContext::new(TaskId::new(), dummy_graph());
    ctx.push_task_event(MetricsEventKind::StartTask).await;

    let counters = metrics().get_counters().await;
    let gauges = metrics().get_gauges().await;

    assert_eq!(
        counters.get(&MetricsKind::StartTask).unwrap().get(),
        Some(1)
    );

    assert_eq!(
        gauges.get(&MetricsKind::RunningTask).unwrap().get(),
        Some(1f64)
    );
}

#[tokio::test]
async fn applyy_metrics_start_service() {
    let ctx = TaskContext::new(TaskId::new(), dummy_graph());
    ctx.push_service_event(
        &Uri::from("test"),
        &ServiceType::default(),
        MetricsEventKind::StartService,
    )
    .await;

    let counters = metrics().get_counters().await;
    let gauges = metrics().get_gauges().await;

    assert_eq!(
        counters.get(&MetricsKind::StartService).unwrap().get(),
        Some(1)
    );

    assert_eq!(
        gauges.get(&MetricsKind::RunningService).unwrap().get(),
        Some(1f64)
    );
}

fn dummy_graph() -> Graph {
    let map = map_value! {
        "name" => "test",
        "services" => seq_value![
            map_value! {
                "type" => "toy.plugin.test.dummy",
                "uri" => "test",
                "wires" => Value::None,
            }
        ]
    };
    Graph::from(map).unwrap()
}
