use crate::dispatch::select::Selector;
pub use crate::optimizer::scheduler::WorkloadHints;
use crate::optimizer::{parallel, gpu};
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

/// An advanced addition operation that accepts performance tuning hints.
/// 
/// Use this if you need fine-grained control over thread counts, power modes,
/// or resource capping (e.g., in background tasks or battery-critical scenarios).
pub fn add_advanced(a: &[f32], b: &[f32], out: &mut [f32], hints: WorkloadHints) {
    let info = SystemInfo::detect();
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

/// Returns the detected system info in v2.0 format.
pub fn get_info() -> CpuInfo {
    SystemInfo::detect().cpu
}

/// Returns the full system information (CPU + GPU).
pub fn get_system_info() -> SystemInfo {
    SystemInfo::detect()
}
