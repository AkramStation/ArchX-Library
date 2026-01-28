pub use crate::optimizer::scheduler::WorkloadHints;
use crate::hardware::SystemInfo;

/// Publicly exposed CPU information.
pub use crate::hardware::CpuInfo;

/// A high-level, CPU-aware addition operation.
/// 
/// This is the "Just Work" API. It automatically detects the best
/// execution path (SIMD, Parallel, or Scalar) based on data size and hardware.
///
/// # Panics
///
/// This function will panic if the input slices `a` and `b` have different lengths, 
/// or if `out` is smaller than the input slices.
///
/// # Example
///
/// ```rust
/// use archx::add;
///
/// let a = vec![1.0; 1000];
/// let b = vec![2.0; 1000];
/// let mut out = vec![0.0; 1000];
///
/// add(&a, &b, &mut out);
/// assert_eq!(out[0], 3.0);
/// ```
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    add_advanced(a, b, out, WorkloadHints::default());
}

pub fn add_advanced(a: &[f32], b: &[f32], out: &mut [f32], hints: WorkloadHints) {
    let mut builder = crate::public_api::ArchX::compute();
    
    if let Some(n) = hints.thread_count {
        builder = builder.max_threads(n);
    }

    let _ = builder.add(a, b, out);
}

/// Returns the detected system info in v3.0 format.
pub fn get_info() -> CpuInfo {
    SystemInfo::detect().cpu
}

/// Returns the full system information (CPU + GPU).
pub fn get_system_info() -> SystemInfo {
    SystemInfo::detect()
}
