#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// AVX implementation of the add operation.
/// 
/// WHY: AVX allows processing 8 floats (256 bits) in a single instruction.
/// This doubles the throughput compared to SSE2.
pub fn add_avx_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Process in chunks of 8 (AVX ymm register size for f32)
    let simd_len = len / 8;
    let main_loop_len = simd_len * 8;

    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: The caller must ensure AVX is supported at runtime.
        unsafe {
            for i in (0..main_loop_len).step_by(8) {
                // Load 8 floats (unaligned)
                let va = _mm256_loadu_ps(a.as_ptr().add(i));
                let vb = _mm256_loadu_ps(b.as_ptr().add(i));
                
                // Add registers
                let vres = _mm256_add_ps(va, vb);
                
                // Store 8 floats (unaligned)
                _mm256_storeu_ps(out.as_mut_ptr().add(i), vres);
            }
        }
    }

    // Handle remainder
    if main_loop_len < len {
        scalar::add_impl(
            &a[main_loop_len..len],
            &b[main_loop_len..len],
            &mut out[main_loop_len..len],
        );
    }
}
