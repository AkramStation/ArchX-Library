use std::time::{Instant, Duration};

pub struct TaskMetrics {
    pub start_time: Instant,
    pub duration: Option<Duration>,
    pub cpu_usage_start: f32,
    pub cpu_usage_end: f32,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            duration: None,
            cpu_usage_start: 0.0, // Should be fetched from detect
            cpu_usage_end: 0.0,
        }
    }

    pub fn complete(&mut self) {
        self.duration = Some(self.start_time.elapsed());
    }
}
