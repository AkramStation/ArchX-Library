use crate::cpu::features::CpuFeatures;
use crate::cpu::arch::CpuArch;
use crate::cpu::bits::Bitness;

/// Aggregated information about the host CPU.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CpuInfo {
    pub arch: CpuArch,
    pub bits: Bitness,
    pub features: CpuFeatures,
    pub cores: usize,
    pub logical_processors: usize,
    pub brand: String,
}

/// Information about a detected GPU device.
#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: Option<String>,
    pub memory_gb: Option<f32>,
    pub api: Option<String>,
    pub is_integrated: bool,
    pub memory_shared: bool,
}

/// Supported GPU APIs for detection records.
#[derive(Debug, Clone, serde::Serialize)]
pub enum GpuApi {
    Cuda,
    OpenCL,
    Vulkan,
    Metal,
    Mock,
}

/// The unified source of truth for system hardware in v3.0.0.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub gpu: Option<GpuInfo>,
    pub available_memory_gb: f64,
}

impl SystemInfo {
    /// Gathers all hardware information.
    pub fn detect() -> Self {
        let state = crate::detect::HardwareState::capture();

        let cpu = CpuInfo {
            arch: state.cpu.arch,
            bits: crate::cpu::bits::detect_bits(),
            features: crate::cpu::features::CpuFeatures::detect(),
            cores: state.cpu.physical_cores,
            logical_processors: state.cpu.logical_threads,
            brand: state.cpu.brand,
        };

        let gpu = state.gpu.map(|g| {
            GpuInfo {
                name: g.name,
                vendor: Some(g.vendor),
                memory_gb: Some(4.0),
                api: Some("Auto-Detected-Driver".to_string()),
                is_integrated: g.is_integrated,
                memory_shared: g.shared_memory,
            }
        });

        Self {
            cpu,
            gpu,
            available_memory_gb: (state.memory.total_kb as f64) / 1024.0 / 1024.0,
        }
    }

    /// Verifies if a dataset of given size can fit into memory.
    pub fn can_handle_dataset(&self, elements: usize) -> bool {
        let required_gb = (elements * 4 * 3) as f64 / 1e9;
        required_gb < self.available_memory_gb * 0.8
    }
}
