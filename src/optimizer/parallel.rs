use std::thread;
use crate::dispatch::select::Selector;
use crate::optimizer::scheduler::{Scheduler, WorkloadHints};

/// Parallel implementation of the add operation with advanced scheduling.
/// 
/// WHY: v0.5 introduces cache-aware chunking and user-provided hints 
/// to maximize throughput on high-core-count and AVX-512 systems.
pub fn add_parallel_impl(a: &[f32], b: &[f32], out: &mut [f32], hints: &WorkloadHints) {
    let _scope = crate::profiling::ProfileScope::new("Parallel Add (Master)", None);
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

    // v1.0 PRODUCTION: Explicitly bound the number of spawns to avoid overhead.
    thread::scope(|s| {
        for i in 0..num_threads {
            let start = i * chunk_size;
            if start >= len { break; }
            let end = (start + chunk_size).min(len);
            
            let a_chunk = &a[start..end];
            let b_chunk = &b[start..end];
            
            // Safety: Disjoint regions verified for v1.0 production parity.
            unsafe {
                let out_ptr = out.as_mut_ptr().add(start);
                let out_slice = std::slice::from_raw_parts_mut(out_ptr, end - start);
                
                s.spawn(move || {
                    let _thread_scope = crate::profiling::ProfileScope::new("Parallel Chunk", Some(start / chunk_size));
                    add_fn(a_chunk, b_chunk, out_slice);
                });
            }
            
            start = end;
        }
    });
}
