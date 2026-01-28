pub mod sse2;
pub mod avx;
pub mod avx2;
pub mod avx512;
pub mod neon;

use crate::cpu::features::CpuFeatures;
use std::sync::OnceLock;

pub type BinaryOpFn = fn(&[f32], &[f32], &mut [f32]);
pub type DotFn = fn(&[f32], &[f32]) -> f32;
pub type ReductionFn = fn(&[f32]) -> f32;

pub struct SimdDispatcher {
    pub add: BinaryOpFn,
    pub sub: BinaryOpFn,
    pub mul: BinaryOpFn,
    pub dot: DotFn,
    pub sum: ReductionFn,
}

static DISPATCHER: OnceLock<SimdDispatcher> = OnceLock::new();

impl SimdDispatcher {
    pub fn global() -> &'static Self {
        DISPATCHER.get_or_init(Self::detect)
    }

    pub fn detect() -> Self {
        let features = CpuFeatures::detect();
        
        // Default Scalar Fallbacks
        let mut dispatcher = Self {
            add: crate::optimizer::scalar::add_impl,
            sub: scalar_sub,
            mul: scalar_mul,
            dot: scalar_dot,
            sum: scalar_sum,
        };

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if features.avx2 {
                dispatcher.add = avx2::add_avx2_impl;
                dispatcher.sub = avx2::sub_avx2_impl;
                dispatcher.mul = avx2::mul_avx2_impl;
                dispatcher.dot = avx2::dot_avx2_impl;
                dispatcher.sum = avx2::sum_avx2_impl;
            } else if features.sse2 {
                dispatcher.add = sse2::add_sse2_impl;
                dispatcher.sub = sse2::sub_sse2_impl;
                dispatcher.mul = sse2::mul_sse2_impl;
                dispatcher.dot = sse2::dot_sse2_impl;
                dispatcher.sum = sse2::sum_sse2_impl;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if features.neon {
                dispatcher.add = neon::add_neon_impl;
                // Add more neon impls as they are developed
            }
        }

        dispatcher
    }
}

// --- Scalar Fallbacks ---

fn scalar_sub(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] - b[i];
    }
}

fn scalar_mul(a: &[f32], b: &[f32], out: &mut [f32]) {
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

fn scalar_dot(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

fn scalar_sum(a: &[f32]) -> f32 {
    a.iter().sum()
}
