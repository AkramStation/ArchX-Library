use crate::optimizer::simd::avx;

/// AVX2 implementation of the add operation.
/// 
/// WHY: While AVX2 mostly adds integer SIMD, having an explicit AVX2 path 
/// allows future-proofing for operations where AVX2/FMA provides a benefit 
/// over plain AVX. For 'add', it currently leverages AVX 256-bit instructions.
pub fn add_avx2_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    // For f32 add, AVX2 uses the same 256-bit instructions as AVX.
    // However, keeping them separate allows adding FMA3 later or 
    // using AVX2-specific gather/scatter if needed.
    avx::add_avx_impl(a, b, out);
}
