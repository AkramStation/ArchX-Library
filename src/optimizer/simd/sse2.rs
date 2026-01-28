#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// SSE2 implementation of the add operation.
pub fn add_sse2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_sse2(a, b, out, |va, vb| unsafe { _mm_add_ps(va, vb) }, scalar::add_impl);
}

/// SSE2 implementation of the sub operation.
pub fn sub_sse2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_sse2(a, b, out, |va, vb| unsafe { _mm_sub_ps(va, vb) }, |sa, sb, so| {
        for i in 0..sa.len() { so[i] = sa[i] - sb[i]; }
    });
}

/// SSE2 implementation of the mul operation.
pub fn mul_sse2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_sse2(a, b, out, |va, vb| unsafe { _mm_mul_ps(va, vb) }, |sa, sb, so| {
        for i in 0..sa.len() { so[i] = sa[i] * sb[i]; }
    });
}

#[inline(always)]
fn bin_op_sse2<F>(a: &[f32], b: &[f32], out: &mut [f32], op: F, fallback: fn(&[f32], &[f32], &mut [f32])) 
where F: Fn(__m128, __m128) -> __m128 
{
    let len = a.len().min(b.len()).min(out.len());
    let unroll_factor = 16;
    let main_loop_len = (len / unroll_factor) * unroll_factor;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        for i in (0..main_loop_len).step_by(unroll_factor) {
            let a_p = a.as_ptr().add(i);
            let b_p = b.as_ptr().add(i);
            let o_p = out.as_mut_ptr().add(i);

            let vres1 = op(_mm_loadu_ps(a_p), _mm_loadu_ps(b_p));
            let vres2 = op(_mm_loadu_ps(a_p.add(4)), _mm_loadu_ps(b_p.add(4)));
            let vres3 = op(_mm_loadu_ps(a_p.add(8)), _mm_loadu_ps(b_p.add(8)));
            let vres4 = op(_mm_loadu_ps(a_p.add(12)), _mm_loadu_ps(b_p.add(12)));

            _mm_storeu_ps(o_p, vres1);
            _mm_storeu_ps(o_p.add(4), vres2);
            _mm_storeu_ps(o_p.add(8), vres3);
            _mm_storeu_ps(o_p.add(12), vres4);
        }
    }

    if main_loop_len < len {
        fallback(&a[main_loop_len..len], &b[main_loop_len..len], &mut out[main_loop_len..len]);
    }
}

/// SSE2 implementation of the dot product.
pub fn dot_sse2_impl(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let unroll_factor = 16;
    let main_loop_len = (len / unroll_factor) * unroll_factor;
    let mut result = 0.0;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        if main_loop_len > 0 {
            let mut v_acc = _mm_setzero_ps();
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let a_p = a.as_ptr().add(i);
                let b_p = b.as_ptr().add(i);

                v_acc = _mm_add_ps(v_acc, _mm_mul_ps(_mm_loadu_ps(a_p), _mm_loadu_ps(b_p)));
                v_acc = _mm_add_ps(v_acc, _mm_mul_ps(_mm_loadu_ps(a_p.add(4)), _mm_loadu_ps(b_p.add(4))));
                v_acc = _mm_add_ps(v_acc, _mm_mul_ps(_mm_loadu_ps(a_p.add(8)), _mm_loadu_ps(b_p.add(8))));
                v_acc = _mm_add_ps(v_acc, _mm_mul_ps(_mm_loadu_ps(a_p.add(12)), _mm_loadu_ps(b_p.add(12))));
            }
            result = hsum_sse(v_acc);
        }
    }

    for i in main_loop_len..len {
        result += a[i] * b[i];
    }
    result
}

/// SSE2 implementation of the sum reduction.
pub fn sum_sse2_impl(a: &[f32]) -> f32 {
    let len = a.len();
    let unroll_factor = 16;
    let main_loop_len = (len / unroll_factor) * unroll_factor;
    let mut result = 0.0;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        if main_loop_len > 0 {
            let mut v_acc = _mm_setzero_ps();
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let p = a.as_ptr().add(i);
                v_acc = _mm_add_ps(v_acc, _mm_loadu_ps(p));
                v_acc = _mm_add_ps(v_acc, _mm_loadu_ps(p.add(4)));
                v_acc = _mm_add_ps(v_acc, _mm_loadu_ps(p.add(8)));
                v_acc = _mm_add_ps(v_acc, _mm_loadu_ps(p.add(12)));
            }
            result = hsum_sse(v_acc);
        }
    }

    for i in main_loop_len..len {
        result += a[i];
    }
    result
}

#[inline(always)]
unsafe fn hsum_sse(v: __m128) -> f32 {
    let v_sum = _mm_add_ps(v, _mm_movehl_ps(v, v));
    let v_sum = _mm_add_ps(v_sum, _mm_shuffle_ps(v_sum, v_sum, 1));
    _mm_cvtss_f32(v_sum)
}
