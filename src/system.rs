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
/// 
/// WHY: v0.5 allows users to specify thread counts or request GPU offloading 
/// for specific workloads.
pub fn add_advanced(a: &[f32], b: &[f32], out: &mut [f32], hints: WorkloadHints) {
    // 1. Check for GPU offloading request
    if hints.prefer_gpu {
        if let Some(backend) = gpu::get_backend() {
            if let Ok(_) = backend.add(a, b, out) {
                return;
            }
        }
    }

    // 2. CPU Execution path
    if a.len() >= PARALLEL_THRESHOLD || hints.thread_count.unwrap_or(0) > 1 {
        parallel::add_parallel_impl(a, b, out, &hints);
    } else {
        Selector::dispatch_add(a, b, out);
    }
}

/// Returns the detected CPU info.
pub fn get_info() -> CpuInfo {
    CpuInfo::detect()
}
