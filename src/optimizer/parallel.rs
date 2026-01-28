use std::thread;
use crate::dispatch::select::Selector;

/// Parallel implementation of the add operation.
/// 
/// WHY: For large workloads, the overhead of spawning threads or managing a 
/// thread pool is outweighed by the performance gain of multi-core execution.
pub fn add_parallel_impl(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Determine the number of threads to use based on available hardware parallelism.
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    
    if num_threads <= 1 {
        Selector::dispatch_add(a, b, out);
        return;
    }

    // Calculate chunk size. We want chunks to be somewhat large to justify threading.
    // Also, we align chunks to 8 elements to stay SIMD-friendly (AVX uses 8-wide).
    let raw_chunk_size = (len + num_threads - 1) / num_threads;
    let chunk_size = (raw_chunk_size + 7) & !7; // Align up to 8

    let add_fn = Selector::get_add_fn();

    thread::scope(|s| {
        let mut start = 0;
        while start < len {
            let end = (start + chunk_size).min(len);
            
            // Sub-divide slices for this thread's chunk.
            let a_chunk = &a[start..end];
            let b_chunk = &b[start..end];
            
            // We need to split 'out' as well. Since we are using pointers or 
            // splitting a mutable slice, we use 'split_at_mut' or unsafe logic.
            // Using unsafe is common in low-level systems for performance 
            // but we must be extremely careful.
            
            // Safety: We ensure that each thread gets a unique, non-overlapping 
            // part of the output buffer.
            unsafe {
                let out_ptr = out.as_mut_ptr().add(start);
                let out_slice = std::slice::from_raw_parts_mut(out_ptr, end - start);
                
                s.spawn(move || {
                    add_fn(a_chunk, b_chunk, out_slice);
                });
            }
            
            start = end;
        }
    });
}
