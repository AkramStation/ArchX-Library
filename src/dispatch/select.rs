use crate::cpu::features::CpuFeatures;
use crate::optimizer::scalar;
use crate::optimizer::simd::sse2;

/// Defines the strategy for execution path selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchPath {
    Scalar,
    SSE2,
    AVX,
    AVX2,
}

/// A selector that decides which implementation path to use based on CPU features.
pub struct Selector;

impl Selector {
    /// Selects the best available execution path for the current CPU.
    /// 
    /// WHY: This abstraction allows us to add more optimized paths (like AVX2) 
    /// without changing the public API or the caller's logic.
    pub fn best_path(features: &CpuFeatures) -> DispatchPath {
        // v0.2: Prioritize SSE2 if available.
        // AVX/AVX2 paths are reserved for future versions.
        
        if features.sse2 {
            DispatchPath::SSE2
        } else {
            DispatchPath::Scalar
        }
    }

    /// Dispatches the 'add' operation to the chosen implementation.
    /// 
    /// WHY: Separation of concerns. The selector knows WHERE to go, 
    /// and this method executes the trip.
    pub fn dispatch_add(a: &[f32], b: &[f32], out: &mut [f32], path: DispatchPath) {
        match path {
            DispatchPath::SSE2 => {
                #[cfg(target_arch = "x86_64")]
                {
                    sse2::add_sse2_impl(a, b, out);
                }
                #[cfg(not(target_arch = "x86_64"))]
                {
                    scalar::add_impl(a, b, out);
                }
            },
            DispatchPath::Scalar => scalar::add_impl(a, b, out),
            // Future paths (AVX/AVX2) will fallback to scalar until implemented
            _ => scalar::add_impl(a, b, out),
        }
    }
}
