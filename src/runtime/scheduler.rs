use rayon::prelude::*;
use crate::optimizer::simd::SimdDispatcher;

pub struct ArchXSched;

impl ArchXSched {
    /// Parallel addition using work-stealing and SIMD.
    pub fn parallel_add(a: &[f32], b: &[f32], out: &mut [f32]) {
        let dispatcher = SimdDispatcher::global();
        let chunk_size = Self::calc_chunk(a.len());
        
        out.par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(i, chunk)| {
                let start = i * chunk_size;
                let end = (start + chunk_size).min(a.len());
                (dispatcher.add)(&a[start..end], &b[start..end], chunk);
            });
    }

    /// Parallel subtraction using work-stealing and SIMD.
    pub fn parallel_sub(a: &[f32], b: &[f32], out: &mut [f32]) {
        let dispatcher = SimdDispatcher::global();
        let chunk_size = Self::calc_chunk(a.len());
        
        out.par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(i, chunk)| {
                let start = i * chunk_size;
                let end = (start + chunk_size).min(a.len());
                (dispatcher.sub)(&a[start..end], &b[start..end], chunk);
            });
    }

    /// Parallel multiplication using work-stealing and SIMD.
    pub fn parallel_mul(a: &[f32], b: &[f32], out: &mut [f32]) {
        let dispatcher = SimdDispatcher::global();
        let chunk_size = Self::calc_chunk(a.len());
        
        out.par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(i, chunk)| {
                let start = i * chunk_size;
                let end = (start + chunk_size).min(a.len());
                (dispatcher.mul)(&a[start..end], &b[start..end], chunk);
            });
    }

    /// Parallel dot product using work-stealing, SIMD, and parallel reduction.
    pub fn parallel_dot(a: &[f32], b: &[f32]) -> f32 {
        let dispatcher = SimdDispatcher::global();
        let chunk_size = Self::calc_chunk(a.len());
        
        a.par_chunks(chunk_size)
            .enumerate()
            .map(|(i, chunk)| {
                let start = i * chunk_size;
                let end = (start + chunk_size).min(b.len());
                (dispatcher.dot)(chunk, &b[start..end])
            })
            .sum()
    }

    /// Parallel sum reduction using work-stealing and SIMD.
    pub fn parallel_sum(a: &[f32]) -> f32 {
        let dispatcher = SimdDispatcher::global();
        let chunk_size = Self::calc_chunk(a.len());
        
        a.par_chunks(chunk_size)
            .map(|chunk| (dispatcher.sum)(chunk))
            .sum()
    }

    #[inline(always)]
    fn calc_chunk(len: usize) -> usize {
        (len / rayon::current_num_threads()).max(1024)
    }
}
