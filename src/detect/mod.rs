pub mod cpu;
pub mod memory;
pub mod gpu_info;
pub mod system;

use cpu::{detect_cpu, CpuDetails};
use memory::{detect_memory, MemoryDetails};
use gpu_info::{detect_gpu, GpuDetails};
use system::{detect_context, SystemContext};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HardwareState {
    pub cpu: CpuDetails,
    pub memory: MemoryDetails,
    pub gpu: Option<GpuDetails>,
    pub context: SystemContext,
    pub timestamp: u64,
}

impl HardwareState {
    pub fn capture() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let cpu = detect_cpu();
        let memory = detect_memory();
        let gpu = detect_gpu(Some(&cpu.brand));
        let context = detect_context();

        Self {
            cpu,
            memory,
            gpu,
            context,
            timestamp: now,
        }
    }
}
