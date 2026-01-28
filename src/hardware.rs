use std::thread;
use crate::cpu::features::CpuFeatures;

/// Aggregated system hardware information for dispatch decisions.
#[derive(Debug, Clone, serde::Serialize)]
pub struct HardwareInfo {
    pub cpu_cores: usize,
    pub logical_processors: usize,
    pub features: CpuFeatures,
    pub suspected_load: f32,
    pub available_memory_gb: f64,
}

impl HardwareInfo {
    pub fn detect() -> Self {
        let logical = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        
        // Physical core estimation (heuristic)
        let cores = (logical / 2).max(1);
        
        Self {
            cpu_cores: cores,
            logical_processors: logical,
            features: CpuFeatures::detect(),
            suspected_load: 0.0,
            available_memory_gb: 8.0, // Baseline for v1.1.1
        }
    }

    pub fn can_handle_dataset(&self, elements: usize) -> bool {
        let required_gb = (elements * 4 * 3) as f64 / 1e9; // 3 f32 vectors (a, b, out)
        required_gb < self.available_memory_gb * 0.8 // 80% safety margin
    }
}
