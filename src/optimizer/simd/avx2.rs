#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use crate::optimizer::scalar;

/// AVX2 implementation of the add operation.
pub fn add_avx2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_avx2(a, b, out, |va, vb| unsafe { _mm256_add_ps(va, vb) }, scalar::add_impl);
}

/// AVX2 implementation of the sub operation.
pub fn sub_avx2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_avx2(a, b, out, |va, vb| unsafe { _mm256_sub_ps(va, vb) }, |sa, sb, so| {
        for i in 0..sa.len() { so[i] = sa[i] - sb[i]; }
    });
}

/// AVX2 implementation of the mul operation.
pub fn mul_avx2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    bin_op_avx2(a, b, out, |va, vb| unsafe { _mm256_mul_ps(va, vb) }, |sa, sb, so| {
        for i in 0..sa.len() { so[i] = sa[i] * sb[i]; }
    });
}

#[inline(always)]
fn bin_op_avx2<F>(a: &[f32], b: &[f32], out: &mut [f32], op: F, fallback: fn(&[f32], &[f32], &mut [f32])) 
where F: Fn(__m256, __m256) -> __m256 
{
    let len = a.len().min(b.len()).min(out.len());
    let unroll_factor = 32;
    let main_loop_len = (len / unroll_factor) * unroll_factor;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        for i in (0..main_loop_len).step_by(unroll_factor) {
            let a_p = a.as_ptr().add(i);
            let b_p = b.as_ptr().add(i);
            let o_p = out.as_mut_ptr().add(i);

            let vres1 = op(_mm256_loadu_ps(a_p), _mm256_loadu_ps(b_p));
            let vres2 = op(_mm256_loadu_ps(a_p.add(8)), _mm256_loadu_ps(b_p.add(8)));
            let vres3 = op(_mm256_loadu_ps(a_p.add(16)), _mm256_loadu_ps(b_p.add(16)));
            let vres4 = op(_mm256_loadu_ps(a_p.add(24)), _mm256_loadu_ps(b_p.add(24)));

            _mm256_storeu_ps(o_p, vres1);
            _mm256_storeu_ps(o_p.add(8), vres2);
            _mm256_storeu_ps(o_p.add(16), vres3);
            _mm256_storeu_ps(o_p.add(24), vres4);
        }
    }

    if main_loop_len < len {
        fallback(&a[main_loop_len..len], &b[main_loop_len..len], &mut out[main_loop_len..len]);
    }
}

/// AVX2 implementation of the dot product.
pub fn dot_avx2_impl(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let unroll_factor = 32;
    let main_loop_len = (len / unroll_factor) * unroll_factor;
    let mut result = 0.0;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        if main_loop_len > 0 {
            let mut v_acc = _mm256_setzero_ps();
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let a_p = a.as_ptr().add(i);
                let b_p = b.as_ptr().add(i);

                v_acc = _mm256_add_ps(v_acc, _mm256_mul_ps(_mm256_loadu_ps(a_p), _mm256_loadu_ps(b_p)));
                v_acc = _mm256_add_ps(v_acc, _mm256_mul_ps(_mm256_loadu_ps(a_p.add(8)), _mm256_loadu_ps(b_p.add(8))));
                v_acc = _mm256_add_ps(v_acc, _mm256_mul_ps(_mm256_loadu_ps(a_p.add(16)), _mm256_loadu_ps(b_p.add(16))));
                v_acc = _mm256_add_ps(v_acc, _mm256_mul_ps(_mm256_loadu_ps(a_p.add(24)), _mm256_loadu_ps(b_p.add(24))));
            }
            result = hsum_avx(v_acc);
        }
    }

    for i in main_loop_len..len {
        result += a[i] * b[i];
    }
    result
}

/// AVX2 implementation of the sum reduction.
pub fn sum_avx2_impl(a: &[f32]) -> f32 {
    let len = a.len();
    let unroll_factor = 32;
    let main_loop_len = (len / unroll_factor) * unroll_factor;
    let mut result = 0.0;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        if main_loop_len > 0 {
            let mut v_acc = _mm256_setzero_ps();
            for i in (0..main_loop_len).step_by(unroll_factor) {
                let p = a.as_ptr().add(i);
                v_acc = _mm256_add_ps(v_acc, _mm256_loadu_ps(p));
                v_acc = _mm256_add_ps(v_acc, _mm256_loadu_ps(p.add(8)));
                v_acc = _mm256_add_ps(v_acc, _mm256_loadu_ps(p.add(16)));
                v_acc = _mm256_add_ps(v_acc, _mm256_loadu_ps(p.add(24)));
            }
            result = hsum_avx(v_acc);
        }
    }

    for i in main_loop_len..len {
        result += a[i];
    }
    result
}

#[inline(always)]
unsafe fn hsum_avx(v: __m256) -> f32 {
    let v_low = _mm256_castps256_ps128(v);
    let v_high = _mm256_extractf128_ps(v, 1);
    let v_sum = _mm_add_ps(v_low, v_high);
    let v_sum = _mm_add_ps(v_sum, _mm_movehl_ps(v_sum, v_sum));
    let v_sum = _mm_add_ps(v_sum, _mm_shuffle_ps(v_sum, v_sum, 1));
    _mm_cvtss_f32(v_sum)
}
