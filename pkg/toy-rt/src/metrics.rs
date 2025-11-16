use std::time::Duration;
use toy_core::metrics::{Counter, Gauge};

pub struct RuntimeMetrics {
    pub(crate) num_workers: usize,
    pub(crate) global_queue_depth: usize,
    pub(crate) io_driver_fd_registered_count: u64,
    pub(crate) io_driver_fd_deregistered_count: u64,
    pub(crate) io_driver_ready_count: u64,

    pub(crate) workers: Vec<WorkerMetrics>,
}

pub struct WorkerMetrics {
    pub(crate) worker: usize,
    pub(crate) worker_park_count: u64,
    pub(crate) worker_noop_count: u64,
    pub(crate) worker_steal_count: u64,
    pub(crate) worker_poll_count: u64,
    pub(crate) worker_total_busy_duration: Duration,
    pub(crate) worker_local_schedule_count: u64,
    pub(crate) worker_overflow_count: u64,
    pub(crate) worker_local_queue_depth: usize,
}

impl RuntimeMetrics {
    pub fn with(metrics: &tokio::runtime::RuntimeMetrics) -> Self {
        let num_workers = metrics.num_workers();
        let mut workers = Vec::with_capacity(num_workers);
        for w in 0..num_workers {
            workers.push(WorkerMetrics::with(metrics, w));
        }

        Self {
            num_workers,
            global_queue_depth: metrics.global_queue_depth(),
            io_driver_fd_registered_count: metrics.io_driver_fd_registered_count(),
            io_driver_fd_deregistered_count: metrics.io_driver_fd_deregistered_count(),
            io_driver_ready_count: metrics.io_driver_ready_count(),
            workers,
        }
    }

    pub fn num_workers(&self) -> usize {
        self.num_workers
    }

    /// Returns the number of tasks currently scheduled in the runtime's injection queue.
    /// returned value may increase or decrease as new tasks are scheduled and processed.
    pub fn global_queue_depth(&self) -> usize {
        self.global_queue_depth
    }

    pub fn io_driver_fd_registered_count(&self) -> u64 {
        self.io_driver_fd_registered_count
    }

    pub fn io_driver_fd_deregistered_count(&self) -> u64 {
        self.io_driver_fd_deregistered_count
    }

    pub fn io_driver_ready_count(&self) -> u64 {
        self.io_driver_ready_count
    }

    pub fn workers(&self) -> &[WorkerMetrics] {
        &self.workers
    }

    pub fn get_counters(&self) -> Vec<(&str, Counter)> {
        vec![
            ("num_workers", Counter::from(self.num_workers as u64)),
            (
                "io_driver_fd_registered_count",
                Counter::from(self.io_driver_fd_registered_count()),
            ),
            (
                "io_driver_fd_deregistered_count",
                Counter::from(self.io_driver_fd_deregistered_count()),
            ),
            (
                "io_driver_ready_count",
                Counter::from(self.io_driver_ready_count()),
            ),
        ]
    }

    pub fn get_gauges(&self) -> Vec<(&str, Gauge)> {
        vec![(
            "global_queue_depth",
            Gauge::from(self.global_queue_depth as f64),
        )]
    }
}

impl WorkerMetrics {
    pub fn with(metrics: &tokio::runtime::RuntimeMetrics, worker: usize) -> Self {
        Self {
            worker,
            worker_park_count: metrics.worker_park_count(worker),
            worker_noop_count: metrics.worker_noop_count(worker),
            worker_steal_count: metrics.worker_steal_count(worker),
            worker_poll_count: metrics.worker_poll_count(worker),
            worker_total_busy_duration: metrics.worker_total_busy_duration(worker),
            worker_local_schedule_count: metrics.worker_local_schedule_count(worker),
            worker_overflow_count: metrics.worker_overflow_count(worker),
            worker_local_queue_depth: metrics.worker_local_queue_depth(worker),
        }
    }

    pub fn worker(&self) -> usize {
        self.worker
    }

    pub fn worker_park_count(&self) -> u64 {
        self.worker_park_count
    }

    pub fn worker_noop_count(&self) -> u64 {
        self.worker_noop_count
    }

    pub fn worker_steal_count(&self) -> u64 {
        self.worker_steal_count
    }

    pub fn worker_poll_count(&self) -> u64 {
        self.worker_poll_count
    }

    pub fn worker_total_busy_duration(&self) -> Duration {
        self.worker_total_busy_duration
    }

    pub fn worker_local_schedule_count(&self) -> u64 {
        self.worker_local_schedule_count
    }

    pub fn worker_overflow_count(&self) -> u64 {
        self.worker_overflow_count
    }

    /// Returns the number of tasks currently scheduled in the given worker's local queue.
    /// As such, the returned value may increase or decrease as new tasks are scheduled and processed.
    pub fn worker_local_queue_depth(&self) -> usize {
        self.worker_local_queue_depth
    }

    pub fn get_counters(&self) -> Vec<(&str, Counter)> {
        vec![
            ("worker_park_count", Counter::from(self.worker_park_count)),
            ("worker_noop_count", Counter::from(self.worker_noop_count)),
            ("worker_steal_count", Counter::from(self.worker_steal_count)),
            ("worker_poll_count", Counter::from(self.worker_poll_count)),
            (
                "worker_local_schedule_count",
                Counter::from(self.worker_local_schedule_count),
            ),
            (
                "worker_overflow_count",
                Counter::from(self.worker_overflow_count),
            ),
        ]
    }

    pub fn get_gauges(&self) -> Vec<(&str, Gauge)> {
        vec![(
            "worker_local_queue_depth",
            Gauge::from(self.worker_local_queue_depth as f64),
        )]
    }
}
