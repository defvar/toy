use toy_core::data::Value;
use toy_core::graph::Graph;
use toy_core::task::{TaskContext, TaskId};
use toy_core::{map_value, seq_value, ServiceType};
use toy_tracing::LogGuard;

pub fn dummy_graph() -> Graph {
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

pub fn dummy_service_type() -> ServiceType {
    ServiceType::new("toy.plugin.test", "dummy").unwrap()
}

pub fn tracing_console() -> Result<LogGuard, std::io::Error> {
    toy_tracing::console()
}

pub fn rust_log_debug() {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
}

pub fn dummy_task_context() -> TaskContext {
    let mut t = TaskContext::new(TaskId::new(), dummy_graph());
    t.set_span(t.info_span().clone());
    t
}
