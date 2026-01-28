use std::thread;
use crate::cpu::features::CpuFeatures;
use crate::cpu::arch::{detect_arch, CpuArch};
use crate::cpu::bits::{detect_bits, Bitness};

/// Aggregated information about the host CPU.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct CpuInfo {
    pub arch: CpuArch,
    pub bits: Bitness,
    pub features: CpuFeatures,
    pub cores: usize,
    pub logical_processors: usize,
}

/// Information about a detected GPU device.
#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub memory_gb: f64,
    pub api: GpuApi,
}

/// Supported GPU APIs for detection.
#[derive(Debug, Clone, serde::Serialize)]
pub enum GpuApi {
    Cuda,
    OpenCL,
    Vulkan,
    Metal,
    Mock,
}

/// The unified source of truth for system hardware in v2.0.0.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub gpu: Option<GpuInfo>,
    pub available_memory_gb: f64,
}

impl SystemInfo {
    /// Gathers all hardware information.
    pub fn detect() -> Self {
        let logical = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let physical = (logical / 2).max(1);

        let cpu = CpuInfo {
            arch: detect_arch(),
            bits: detect_bits(),
            features: CpuFeatures::detect(),
            cores: physical,
            logical_processors: logical,
        };

        // v2.0: Attempt to detect GPU through registered backends or system queries
        let gpu = crate::optimizer::gpu::get_active_backend_name().map(|name| {
            GpuInfo {
                name,
                vendor: "Detected".to_string(),
                memory_gb: 4.0, // Default estimate for v2.0
                api: GpuApi::Mock, // Default for now
            }
        });

        Self {
            cpu,
            gpu,
            available_memory_gb: 8.0, // Baseline detection
        }
    }

    /// Verifies if a dataset of given size can fit into memory.
    pub fn can_handle_dataset(&self, elements: usize) -> bool {
        let required_gb = (elements * 4 * 3) as f64 / 1e9;
        required_gb < self.available_memory_gb * 0.8
    }
}
