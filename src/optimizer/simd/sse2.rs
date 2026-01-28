#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// SSE2 implementation of the add operation.
/// 
/// WHY: SSE2 allows processing 4 floats (128 bits) in a single instruction.
/// This implementation uses unaligned loads/stores to ensure safety regardless 
/// of slice alignment, which is critical for a general-purpose library.
pub fn add_sse2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Process in chunks of 4 (SSE2 xmm register size for f32)
    let simd_len = len / 4;
    let main_loop_len = simd_len * 4;

    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: This block is only entered if the target arch is x86_64.
        // The caller (dispatch system) must also verify SSE2 support at runtime.
        unsafe {
            for i in (0..main_loop_len).step_by(4) {
                // Load 4 floats from 'a' and 'b' (unaligned)
                let va = _mm_loadu_ps(a.as_ptr().add(i));
                let vb = _mm_loadu_ps(b.as_ptr().add(i));
                
                // Add registers
                let vres = _mm_add_ps(va, vb);
                
                // Store 4 floats to 'out' (unaligned)
                _mm_storeu_ps(out.as_mut_ptr().add(i), vres);
            }
        }
    }

    // Handle remainder using scalar fallback
    if main_loop_len < len {
        scalar::add_impl(
            &a[main_loop_len..len],
            &b[main_loop_len..len],
            &mut out[main_loop_len..len],
        );
    }
}
