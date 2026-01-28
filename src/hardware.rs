use crate::cpu::features::CpuFeatures;
use std::thread;

/// Detailed information about the system hardware.
/// 
/// WHY: v0.9 uses this to make smarter path decisions (e.g. avoiding GPU 
/// for small data or capping threads on E-cores).
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
        
        // Physical core estimation (clamped to at least 1)
        // Note: In a production v1.0, we would use a crate like 'sysinfo' or 'num_cpus' 
        // for real topology, but for this standalone impl we use logical/2.
        let cores = (logical / 2).max(1);
        
        Self {
            cpu_cores: cores,
            logical_processors: logical,
            features: CpuFeatures::detect(),
            suspected_load: 0.0,
            available_memory_gb: 8.0, // Placeholder: v1.0 would poll OS metrics
        }
    }

    pub fn can_handle_dataset(&self, elements: usize) -> bool {
        let required_gb = (elements * 4 * 3) as f64 / 1e9; // 3 f32 vectors
        required_gb < self.available_memory_gb * 0.8 // 80% safety margin
    }
}
