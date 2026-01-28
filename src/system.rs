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

/// A high-level, CPU-aware addition operation.
/// 
/// This function automatically detects the best implementation for the current CPU.
/// 
/// # Arguments
/// * `a` - First input slice
/// * `b` - Second input slice
/// * `out` - Output slice to store results (a + b)
/// 
/// WHY: This is the primary entry point for users. It hides all complexity
/// of SIMD detection and dispatching.
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    // 1. Detect CPU features (ideally this could be cached for performance)
    let features = CpuFeatures::detect();
    
    // 2. Select best execution path
    let path = Selector::best_path(&features);
    
    // 3. Dispatch to implementation
    Selector::dispatch_add(a, b, out, path);
}

/// Returns the detected CPU info.
pub fn get_info() -> CpuInfo {
    CpuInfo::detect()
}
