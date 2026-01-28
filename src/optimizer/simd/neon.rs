#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
use crate::optimizer::scalar;

/// AArch64 NEON implementation of the add operation with v2.0 loop unrolling.
/// 
/// Process 16 floats per iteration (4x q registers). This provides 
/// high performance on Apple Silicon and modern ARM64 servers.
pub fn add_neon_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Process in chunks of 16 (4x 128-bit registers)
    let unroll_factor = 16;
    let simd_len = len / unroll_factor;
    let main_loop_len = simd_len * unroll_factor;

    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let a_ptr = a.as_ptr().add(i);
                let b_ptr = b.as_ptr().add(i);
                let out_ptr = out.as_mut_ptr().add(i);

                // Quad-vector load (Neon)
                let va1 = vld1q_f32(a_ptr);
                let va2 = vld1q_f32(a_ptr.add(4));
                let va3 = vld1q_f32(a_ptr.add(8));
                let va4 = vld1q_f32(a_ptr.add(12));
                
                let vb1 = vld1q_f32(b_ptr);
                let vb2 = vld1q_f32(b_ptr.add(4));
                let vb3 = vld1q_f32(b_ptr.add(8));
                let vb4 = vld1q_f32(b_ptr.add(12));
                
                let vres1 = vaddq_f32(va1, vb1);
                let vres2 = vaddq_f32(va2, vb2);
                let vres3 = vaddq_f32(va3, vb3);
                let vres4 = vaddq_f32(va4, vb4);
                
                vst1q_f32(out_ptr, vres1);
                vst1q_f32(out_ptr.add(4), vres2);
                vst1q_f32(out_ptr.add(8), vres3);
                vst1q_f32(out_ptr.add(4).add(8), vres4); // out_ptr.add(12)
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
