use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Represents a single execution metric in v2.0.0.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Metric {
    pub name: String,
    pub subsystem: String, // "SIMD", "Parallel", "GPU", etc.
    pub device: String,    // "CPU", "GPU:0", etc.
    pub duration: Duration,
    pub thread_id: Option<usize>,
    pub metadata: HashMap<String, String>,
}

/// Global registry for execution profiling.
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

    /// Exports metrics as a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.get_snapshot()).unwrap_or_else(|_| "[]".into())
    }

    /// Exports metrics as a professional CSV string with device info.
    pub fn to_csv(&self) -> String {
        let snapshot = self.get_snapshot();
        let mut csv = String::from("name,subsystem,device,duration_ms,thread_id\n");
        for m in snapshot {
            csv.push_str(&format!("{},{},{},{:.4},{:?}\n", 
                m.name, m.subsystem, m.device, m.duration.as_secs_f64() * 1000.0, m.thread_id));
        }
        csv
    }

    /// Prints a professional, human-readable performance summary.
    pub fn print_summary(&self) {
        let snapshot = self.get_snapshot();
        if snapshot.is_empty() {
            println!("\x1b[33m[ArchX Profiler]\x1b[0m No metrics collected.");
            return;
        }

        println!("\n\x1b[1;36m┌── ArchX Sovereign v2.0 Profile ───────────────────────────────┐\x1b[0m");
        println!("│ \x1b[1m{:<20}\x1b[0m │ \x1b[1m{:<10}\x1b[0m │ \x1b[1m{:<10}\x1b[0m │ \x1b[1m{:<12}\x1b[0m │", "Task", "Subsystem", "Device", "Time (ms)");
        println!("├──────────────────────┼────────────┼────────────┼──────────────┤");
        for m in snapshot {
            println!("│ {:<20} │ {:<10} │ {:<10} │ {:>10.4} ms │", 
                m.name, 
                m.subsystem,
                m.device,
                m.duration.as_secs_f64() * 1000.0
            );
        }
        println!("\x1b[1;36m└──────────────────────┴────────────┴────────────┴──────────────┘\x1b[0m\n");
    }
}

static GLOBAL_PROFILER: OnceLock<Profiler> = OnceLock::new();

pub fn get_profiler() -> &'static Profiler {
    GLOBAL_PROFILER.get_or_init(Profiler::new)
}

/// A scope-based timer for profiling.
pub struct ProfileScope {
    name: String,
    subsystem: String,
    device: String,
    start: Instant,
    thread_id: Option<usize>,
}

impl ProfileScope {
    pub fn new(name: &str, subsystem: &str, device: &str, thread_id: Option<usize>) -> Self {
        Self {
            name: name.to_string(),
            subsystem: subsystem.to_string(),
            device: device.to_string(),
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
            subsystem: self.subsystem.clone(),
            device: self.device.clone(),
            duration,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        });
    }
}
