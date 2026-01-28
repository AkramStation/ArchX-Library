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

    // v1.1: Pre-calculate pointers to minimize loop overhead
    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();

    thread::scope(|s| {
        for i in 0..num_threads {
            let offset = i * chunk_size;
            if offset >= len { break; }
            let count = (len - offset).min(chunk_size);
            
            unsafe {
                let ca = std::slice::from_raw_parts(a_ptr.add(offset), count);
                let cb = std::slice::from_raw_parts(b_ptr.add(offset), count);
                let out_ptr = out.as_mut_ptr().add(offset);
                let co = std::slice::from_raw_parts_mut(out_ptr, count);
                
                s.spawn(move || {
                    let _thread_scope = crate::profiling::ProfileScope::new("Parallel Chunk", Some(i));
                    add_fn(ca, cb, co);
                });
            }
        }
    });
}
