pub mod sse2;
pub mod avx;
pub mod avx2;
pub mod avx512;
pub mod neon;

use crate::cpu::features::CpuFeatures;

pub type AddFn = fn(&[f32], &[f32], &mut [f32]);

pub struct SimdDispatcher {
    pub add: AddFn,
}

impl SimdDispatcher {
    pub fn detect() -> Self {
        let features = CpuFeatures::detect();
        
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if features.avx512f {
                return Self { add: avx512::add_avx512_impl };
            }
            if features.avx2 {
                return Self { add: avx2::add_avx2_impl };
            }
            if features.avx {
                return Self { add: avx::add_avx_impl };
            }
            if features.sse2 {
                return Self { add: sse2::add_sse2_impl };
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if features.neon {
                return Self { add: neon::add_neon_impl };
            }
        }

        Self { add: crate::optimizer::scalar::add_impl }
    }
}
