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

use crate::optimizer::parallel;

/// Threshold for switching from single-threaded to multi-threaded execution.
/// 
/// WHY: Threading has overhead (context switching, synchronization). 
/// For small vectors, single-threaded SIMD is much faster.
const PARALLEL_THRESHOLD: usize = 128_000;

/// A high-level, CPU-aware addition operation.
/// 
/// This function automatically detects the best implementation for the current CPU
/// and decides whether to use parallel processing based on the data size.
/// 
/// # Arguments
/// * `a` - First input slice
/// * `b` - Second input slice
/// * `out` - Output slice to store results (a + b)
/// 
/// WHY: This is the primary entry point for users. It hides all complexity
/// of SIMD detection, dispatching, and thread management.
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    if a.len() >= PARALLEL_THRESHOLD {
        parallel::add_parallel_impl(a, b, out);
    } else {
        // Dispatch using the optimized single-threaded SIMD system
        Selector::dispatch_add(a, b, out);
    }
}

/// Returns the detected CPU info.
pub fn get_info() -> CpuInfo {
    CpuInfo::detect()
}
