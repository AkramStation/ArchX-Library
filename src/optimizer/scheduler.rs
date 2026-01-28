#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub enum PowerMode {
    #[default]
    Balanced,
    HighPerformance,
    PowerSaving,
}

/// Advanced performance tuning hints for ArchX operations.
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct WorkloadHints {
    pub thread_count: Option<usize>,
    pub min_chunk_size: Option<usize>,
    pub prefer_gpu: bool,
    /// v0.9: Max percentage of available cores to use (0.0 to 1.0)
    pub max_cpu_usage: Option<f32>,
    /// v0.9: Operational mode for thermal/battery efficiency
    pub power_mode: PowerMode,
}

/// Cache-aware scheduler for partitioning workloads.
pub struct Scheduler;

impl Scheduler {
    /// Calculates the optimal chunk size for a given workload.
    /// 
    /// v0.7 Adaptive Logic:
    /// - For very small workloads, use single-threaded execution.
    /// - For large workloads, partition based on available parallelism 
    ///   and cache boundaries.
    pub fn calculate_chunk_size(len: usize, num_threads: usize, hints: &WorkloadHints) -> usize {
        if len < 8192 && hints.thread_count.is_none() {
            return len; // Too small for threading overhead
        }

        if let Some(min) = hints.min_chunk_size {
            return (min + 15) & !15;
        }

        // Adaptive heuristic: If system load is high (simulated), increase chunk size 
        // to reduce task spawning frequency.
        let base_chunk = len.div_ceil(num_threads);
        
        // Ensure floor (e.g., 8192 elements) to satisfy cache-line utilization
        let floor = 8192;
        let chunk_size = base_chunk.max(floor);
        
        // Align to 16 elements (64 bytes)
        (chunk_size + 15) & !15
    }
}
