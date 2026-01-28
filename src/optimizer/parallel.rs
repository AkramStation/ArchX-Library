use std::thread;
use crate::dispatch::select::Selector;
use crate::optimizer::scheduler::{Scheduler, WorkloadHints};

/// Parallel implementation of the add operation with advanced scheduling.
/// 
/// WHY: v0.5 introduces cache-aware chunking and user-provided hints 
/// to maximize throughput on high-core-count and AVX-512 systems.
pub fn add_parallel_impl(a: &[f32], b: &[f32], out: &mut [f32], hints: &WorkloadHints) {
    let len = a.len().min(b.len()).min(out.len());
    
    // Determine the number of threads.
    let num_threads = hints.thread_count.unwrap_or_else(|| {
        thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
    });
    
    if num_threads <= 1 {
        Selector::dispatch_add(a, b, out);
        return;
    }

    // Use the cache-aware scheduler to calculate optimal chunk size.
    let chunk_size = Scheduler::calculate_chunk_size(len, num_threads, hints);
    let add_fn = Selector::get_add_fn();

    thread::scope(|s| {
        let mut start = 0;
        while start < len {
            let end = (start + chunk_size).min(len);
            
            let a_chunk = &a[start..end];
            let b_chunk = &b[start..end];
            
            // Safety: Each thread writes to a disjoin region of the output buffer.
            // Alignment to 16 elements (64 bytes) ensures no false sharing on 
            // modern CPU cache lines.
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
