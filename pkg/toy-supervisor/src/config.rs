use crate::exporters::{EventExporter, MetricsExporter};

pub trait SupervisorConfig {
    type EventExporter: EventExporter;
    type MetricsExporter: MetricsExporter;

    fn heart_beat_interval_mills(&self) -> u64;

    fn event_export_interval_mills(&self) -> u64;

    fn metrics_export_interval_mills(&self) -> u64 {
        self.event_export_interval_mills()
    }

    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;

    fn pub_path(&self) -> String;

    fn metrics_exporter(&self) -> Self::MetricsExporter;

    fn event_exporter(&self) -> Self::EventExporter;
}
