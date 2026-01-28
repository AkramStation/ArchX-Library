/// Configuration hints for workload execution.
/// 
/// WHY: Advanced users may want to tune the thread count or chunk size 
/// based on their specific hardware or workload characteristics.
#[derive(Debug, Clone, Copy, Default)]
pub struct WorkloadHints {
    /// Desired number of threads (0 means auto-detect).
    pub thread_count: Option<usize>,
    /// Minimum elements per chunk.
    pub min_chunk_size: Option<usize>,
    /// Whether to attempt GPU offload if possible.
    pub prefer_gpu: bool,
}

/// Cache-aware scheduler for partitioning workloads.
pub struct Scheduler;

impl Scheduler {
    /// Calculates the optimal chunk size for a given workload.
    /// 
    /// WHY: We want chunks to be large enough to justify thread overhead 
    /// but small enough to fit in L1/L2 caches and allow even distribution.
    /// Aligning to 64 bytes (16 floats) avoids false sharing and is SIMD-optimal.
    pub fn calculate_chunk_size(len: usize, num_threads: usize, hints: &WorkloadHints) -> usize {
        if let Some(min) = hints.min_chunk_size {
            return (min + 15) & !15;
        }

        // Base heuristic: Divide work evenly but don't go below a reasonable floor.
        let base_chunk = (len + num_threads - 1) / num_threads;
        
        // Ensure floor (e.g., 8192 elements) to satisfy cache-line utilization
        // and prevent fine-grained task spawning overhead.
        let floor = 8192;
        let chunk_size = base_chunk.max(floor);
        
        // Align to 16 elements (64 bytes)
        (chunk_size + 15) & !15
    }
}
