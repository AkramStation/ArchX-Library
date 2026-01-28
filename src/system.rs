use crate::cpu::{
    arch::{detect_arch, CpuArch},
    bits::{detect_bits, Bitness},
    features::CpuFeatures,
};
use crate::dispatch::select::Selector;

/// Aggregated information about the host CPU.
/// 
/// WHY: Providing a single struct for CPU info makes it easy for users 
/// to log or inspect the environment their code is running in.
#[derive(Debug, Clone, Copy)]
pub struct CpuInfo {
    pub arch: CpuArch,
    pub bits: Bitness,
    pub features: CpuFeatures,
}

impl CpuInfo {
    /// Gathers all CPU information.
    pub fn detect() -> Self {
        Self {
            arch: detect_arch(),
            bits: detect_bits(),
            features: CpuFeatures::detect(),
        }
    }
}

pub use crate::optimizer::scheduler::WorkloadHints;
use crate::optimizer::{parallel, gpu};

/// Threshold for switching from single-threaded to multi-threaded execution.
const PARALLEL_THRESHOLD: usize = 128_000;

/// A high-level, CPU-aware addition operation.
/// 
/// This function uses default heuristics for optimal performance. 
/// For large workloads, it automatically employs parallel processing.
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    add_advanced(a, b, out, WorkloadHints::default());
}

/// An advanced addition operation that accepts performance tuning hints.
pub fn add_advanced(a: &[f32], b: &[f32], out: &mut [f32], hints: WorkloadHints) {
    let info = crate::hardware::HardwareInfo::detect();
    let strategy = crate::adaptive::AdaptiveEngine::choose_strategy(a.len(), &hints, &info);

    match strategy {
        crate::adaptive::Strategy::GpuOffload => {
            let gpu_result = gpu::with_backend(|backend| backend.add(a, b, out));
            if let Some(Ok(_)) = gpu_result { return; }
            // Fallback to parallel if GPU fails
            parallel::add_parallel_impl(a, b, out, &hints);
        }
        crate::adaptive::Strategy::ParallelSimd(n) => {
            let mut active_hints = hints.clone();
            active_hints.thread_count = Some(n);
            parallel::add_parallel_impl(a, b, out, &active_hints);
        }
        crate::adaptive::Strategy::SingleThreadSimd => {
            Selector::dispatch_add(a, b, out);
        }
        crate::adaptive::Strategy::ScalarFallback => {
            crate::optimizer::scalar::add_impl(a, b, out);
        }
    }
}

/// Returns the detected CPU info.
pub fn get_info() -> CpuInfo {
    CpuInfo::detect()
}
