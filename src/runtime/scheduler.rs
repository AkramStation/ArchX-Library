use rayon::prelude::*;
use crate::optimizer::simd::SimdDispatcher;

pub struct ArchXSched;

impl ArchXSched {
    /// High-performance parallel addition using work-stealing and SIMD.
    pub fn parallel_add(a: &[f32], b: &[f32], out: &mut [f32]) {
        let dispatcher = SimdDispatcher::detect();
        let chunk_size = (a.len() / rayon::current_num_threads()).max(1024);
        
        out.par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(i, chunk)| {
                let start = i * chunk_size;
                let end = (start + chunk_size).min(a.len());
                let slice_a = &a[start..end];
                let slice_b = &b[start..end];
                (dispatcher.add)(slice_a, slice_b, chunk);
            });
    }
}
