use crate::cpu::features::CpuFeatures;
use crate::optimizer::scalar;

/// Defines the strategy for execution path selection.
#[derive(Debug, Clone, Copy)]
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
        // v0.1: Default strictly to Scalar for now, but the structure 
        // demonstrates the intended dispatch logic.
        
        if features.avx2 {
            // Placeholder: In v0.2, this would return DispatchPath::AVX2
            // For now, we fallback to scalar as requested.
            DispatchPath::Scalar 
        } else if features.avx {
            DispatchPath::Scalar
        } else if features.sse2 {
            DispatchPath::Scalar
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
            DispatchPath::Scalar => scalar::add_impl(a, b, out),
            // Future paths will be added here
            _ => scalar::add_impl(a, b, out),
        }
    }
}
