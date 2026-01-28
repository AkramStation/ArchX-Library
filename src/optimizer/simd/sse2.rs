#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// SSE2 implementation of the add operation with v2.0 loop unrolling.
/// 
/// Process 16 floats per iteration (4x xmm registers). This provides 
/// a strong performance baseline for older 64-bit systems.
pub fn add_sse2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Process in chunks of 16 (4x SSE2 registers)
    let unroll_factor = 16;
    let simd_len = len / unroll_factor;
    let main_loop_len = simd_len * unroll_factor;

    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let a_ptr = a.as_ptr().add(i);
                let b_ptr = b.as_ptr().add(i);
                let out_ptr = out.as_mut_ptr().add(i);

                // Quad-vector load
                let va1 = _mm_loadu_ps(a_ptr);
                let va2 = _mm_loadu_ps(a_ptr.add(4));
                let va3 = _mm_loadu_ps(a_ptr.add(8));
                let va4 = _mm_loadu_ps(a_ptr.add(12));
                
                let vb1 = _mm_loadu_ps(b_ptr);
                let vb2 = _mm_loadu_ps(b_ptr.add(4));
                let vb3 = _mm_loadu_ps(b_ptr.add(8));
                let vb4 = _mm_loadu_ps(b_ptr.add(12));
                
                let vres1 = _mm_add_ps(va1, vb1);
                let vres2 = _mm_add_ps(va2, vb2);
                let vres3 = _mm_add_ps(va3, vb3);
                let vres4 = _mm_add_ps(va4, vb4);
                
                _mm_storeu_ps(out_ptr, vres1);
                _mm_storeu_ps(out_ptr.add(4), vres2);
                _mm_storeu_ps(out_ptr.add(8), vres3);
                _mm_storeu_ps(out_ptr.add(12), vres4);
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
