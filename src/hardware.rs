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
    pub suspected_load: f32, // 0.0 to 1.0 (estimated)
}

impl HardwareInfo {
    pub fn detect() -> Self {
        let logical = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        
        Self {
            cpu_cores: logical / 2, // Simple heuristic for physical cores
            logical_processors: logical,
            features: CpuFeatures::detect(),
            suspected_load: 0.0, // Initial load is zero (needs polling logic)
        }
    }
}
