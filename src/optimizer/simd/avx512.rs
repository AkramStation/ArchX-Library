#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// AVX-512 implementation of the add operation.
/// 
/// WHY: AVX-512 allows processing 16 floats (512 bits) in a single instruction.
/// This doubles the throughput compared to AVX/AVX2 on supported hardware.
pub fn add_avx512_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Process in chunks of 16 (AVX-512 zmm register size for f32)
    let simd_len = len / 16;
    let main_loop_len = simd_len * 16;

    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: The caller must ensure AVX-512F is supported at runtime.
        unsafe {
            for i in (0..main_loop_len).step_by(16) {
                // Load 16 floats (unaligned)
                let va = _mm512_loadu_ps(a.as_ptr().add(i));
                let vb = _mm512_loadu_ps(b.as_ptr().add(i));
                
                // Add registers
                let vres = _mm512_add_ps(va, vb);
                
                // Store 16 floats (unaligned)
                _mm512_storeu_ps(out.as_mut_ptr().add(i), vres);
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
