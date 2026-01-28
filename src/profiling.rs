use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Represents a single execution metric.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Metric {
    pub name: String,
    pub duration: Duration,
    pub thread_id: Option<usize>,
    pub metadata: HashMap<String, String>,
}

/// Global registry for execution profiling.
/// 
/// WHY: v0.8 focuses on full verification and transparency. 
/// This registry allows collecting metrics from deep within SIMD/GPU paths.
pub struct Profiler {
    metrics: Mutex<Vec<Metric>>,
    is_enabled: Mutex<bool>,
}

impl Profiler {
    fn new() -> Self {
        Self {
            metrics: Mutex::new(Vec::new()),
            is_enabled: Mutex::new(false),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut lock) = self.is_enabled.lock() {
            *lock = enabled;
        }
    }

    pub fn record(&self, metric: Metric) {
        if let Ok(enabled) = self.is_enabled.lock() {
            if !*enabled { return; }
        }
        if let Ok(mut lock) = self.metrics.lock() {
            lock.push(metric);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut lock) = self.metrics.lock() {
            lock.clear();
        }
    }

    pub fn get_snapshot(&self) -> Vec<Metric> {
        if let Ok(lock) = self.metrics.lock() {
            lock.clone()
        } else {
            Vec::new()
        }
    }
}

static GLOBAL_PROFILER: OnceLock<Profiler> = OnceLock::new();

pub fn get_profiler() -> &'static Profiler {
    GLOBAL_PROFILER.get_or_init(Profiler::new)
}

/// A scope-based timer for profiling.
pub struct ProfileScope {
    name: String,
    start: Instant,
    thread_id: Option<usize>,
}

impl ProfileScope {
    pub fn new(name: &str, thread_id: Option<usize>) -> Self {
        Self {
            name: name.to_string(),
            start: Instant::now(),
            thread_id,
        }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        get_profiler().record(Metric {
            name: self.name.clone(),
            duration,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        });
    }
}
