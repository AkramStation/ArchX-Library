use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Represents a single execution metric in v3.0.0.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Metric {
    pub name: String,
    pub backend: String,   // "SIMD", "Vulkan", "OpenCL"
    pub device: String,    // "CPU", "iGPU", "dGPU"
    pub duration: Duration,
    pub energy_estimate: f32, // estimated power draw
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
        let mut csv = String::from("p_name,backend,device,duration_ms,energy_est,thread_id\n");
        for m in snapshot {
            csv.push_str(&format!("{},{},{},{:.4},{:.2},{:?}\n", 
                m.name, m.backend, m.device, m.duration.as_secs_f64() * 1000.0, m.energy_estimate, m.thread_id));
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

        println!("\n\x1b[1;36m┌── ArchX Sovereign v3.0 Profile ───────────────────────────────┐\x1b[0m");
        println!("│ \x1b[1m{:<18}\x1b[0m │ \x1b[1m{:<10}\x1b[0m │ \x1b[1m{:<10}\x1b[0m │ \x1b[1m{:<12}\x1b[0m │", "Task", "Device", "Backend", "Time (ms)");
        println!("├────────────────────┼────────────┼───────────┼────────────────┤");
        for m in snapshot {
            println!("│ {:<18} │ {:<10} │ {:<9} │ {:>10.4} ms │", 
                m.name, 
                m.device,
                m.backend,
                m.duration.as_secs_f64() * 1000.0
            );
        }
        println!("\x1b[1;36m└────────────────────┴────────────┴───────────┴────────────────┘\x1b[0m\n");
    }
}

static GLOBAL_PROFILER: OnceLock<Profiler> = OnceLock::new();

pub fn get_profiler() -> &'static Profiler {
    GLOBAL_PROFILER.get_or_init(Profiler::new)
}

/// A scope-based timer for profiling.
pub struct ProfileScope {
    name: String,
    backend: String,
    device: String,
    start: Instant,
    thread_id: Option<usize>,
}

impl ProfileScope {
    pub fn new(name: &str, device: &str, backend: &str) -> Self {
        Self {
            name: name.to_string(),
            device: device.to_string(),
            backend: backend.to_string(),
            start: Instant::now(),
            thread_id: None,
        }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        // Estimated energy profile (placeholder logic)
        // Energy = Duration * BasePower (CPU ~45W, iGPU ~15W)
        let energy_multiplier = if self.device.contains("GPU") { 15.0 } else { 45.0 };
        let energy_estimate = duration.as_secs_f32() * energy_multiplier;

        get_profiler().record(Metric {
            name: self.name.clone(),
            backend: self.backend.clone(),
            device: self.device.clone(),
            duration,
            energy_estimate,
            thread_id: self.thread_id,
            metadata: HashMap::new(),
        });
    }
}
