use std::sync::OnceLock;
use crate::cpu::features::CpuFeatures;
use crate::optimizer::scalar;
use crate::optimizer::simd::{sse2, avx, avx2};

/// Type definition for the optimized 'add' function.
type AddFn = fn(&[f32], &[f32], &mut [f32]);

/// Defines the strategy for execution path selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchPath {
    Scalar,
    SSE2,
    AVX,
    AVX2,
    AVX512,
}

/// A selector that decides which implementation path to use based on CPU features.
/// 
/// Refactored in v0.5 to support AVX-512.
pub struct Selector;

static CACHED_ADD_FN: OnceLock<AddFn> = OnceLock::new();

impl Selector {
    /// Selects the best available execution path for the current CPU.
    pub fn best_path(features: &CpuFeatures) -> DispatchPath {
        if features.avx512f {
            DispatchPath::AVX512
        } else if features.avx2 {
            DispatchPath::AVX2
        } else if features.avx {
            DispatchPath::AVX
        } else if features.sse2 {
            DispatchPath::SSE2
        } else {
            DispatchPath::Scalar
        }
    }

    /// Returns the cached, optimal function pointer for the 'add' operation.
    /// 
    /// WHY: This prevents branching and feature detection overhead in hot loops.
    pub fn get_add_fn() -> AddFn {
        *CACHED_ADD_FN.get_or_init(|| {
            let features = CpuFeatures::detect();
            match Self::best_path(&features) {
                DispatchPath::AVX512 => {
                    #[cfg(target_arch = "x86_64")]
                    { crate::optimizer::simd::avx512::add_avx512_impl }
                    #[cfg(not(target_arch = "x86_64"))]
                    { scalar::add_impl }
                }
                DispatchPath::AVX2 => {
                    #[cfg(target_arch = "x86_64")]
                    { avx2::add_avx2_impl }
                    #[cfg(not(target_arch = "x86_64"))]
                    { scalar::add_impl }
                }
                DispatchPath::AVX => {
                    #[cfg(target_arch = "x86_64")]
                    { avx::add_avx_impl }
                    #[cfg(not(target_arch = "x86_64"))]
                    { scalar::add_impl }
                }
                DispatchPath::SSE2 => {
                    #[cfg(target_arch = "x86_64")]
                    { sse2::add_sse2_impl }
                    #[cfg(not(target_arch = "x86_64"))]
                    { scalar::add_impl }
                }
                DispatchPath::Scalar => scalar::add_impl,
            }
        })
    }

    /// Dispatches the 'add' operation using the cached optimal path.
    pub fn dispatch_add(a: &[f32], b: &[f32], out: &mut [f32]) {
        // v0.7: Check extensible plugin systems first
        if crate::plugin::try_plugins(a, b, out) {
            return;
        }

        let func = Self::get_add_fn();
        func(a, b, out);
    }
}
