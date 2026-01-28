use crate::hardware::SystemInfo;
use crate::optimizer::scheduler::{WorkloadHints, PowerMode};

pub use crate::decision::Policy;

/// The strategy chosen by the adaptive engine in v2.0.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    ScalarFallback,
    SingleThreadSimd,
    ParallelSimd(usize), // Active thread count
    GpuOffload,
    Hybrid, // New in v2.1
}

/// A smart engine that decides the best compute path at runtime.
pub struct AdaptiveEngine;

impl AdaptiveEngine {
    /// Decides the best strategy based on data size and system state.
    pub fn choose_strategy(len: usize, hints: &WorkloadHints, info: &SystemInfo) -> Strategy {
        // v2.0 Production Guard: Memory Check
        if !info.can_handle_dataset(len) {
            println!("[ArchX v2.0 WARNING] Dataset size {} exceeds safe memory limits. Forcing SingleThreadSimd.", len);
            return Strategy::SingleThreadSimd;
        }

        // 1. GPU Path - Refined for Sovereign Upgrade
        // We only offload if a GPU is detected and the dataset is large enough to 
        // justify the PCIe overhead (approx 250k elements for AVX-2, 1M for AVX-512).
        let _gpu_threshold = if info.cpu.features.avx512f { 1_000_000 } else { 250_000 };
        if len > 1_000_000 && hints.prefer_gpu && info.gpu.is_some() {
            return Strategy::Hybrid;
        }

        if len > 250_000 && hints.prefer_gpu && info.gpu.is_some() {
            return Strategy::GpuOffload;
        }

        // 2. Extremely small datasets should avoid SIMD/Parallel setup costs.
        // v2.0 Threshold: 1024 elements (cache-line optimized)
        if len < 1024 {
            return Strategy::ScalarFallback;
        }

        // 3. Power & Scaling Heuristics
        let base_threads = match hints.power_mode {
            PowerMode::PowerSaving => info.cpu.cores,
            _ => info.cpu.logical_processors,
        };

        let mut target_threads = if let Some(cap) = hints.max_cpu_usage {
            (info.cpu.logical_processors as f32 * cap).max(1.0) as usize
        } else {
            base_threads
        };
        
        // Manual override guard
        if let Some(manual) = hints.thread_count {
            target_threads = target_threads.min(manual);
        }

        match hints.power_mode {
            PowerMode::PowerSaving => {
                if len < 1_000_000 {
                    Strategy::SingleThreadSimd
                } else {
                    Strategy::ParallelSimd(target_threads.min(info.cpu.cores))
                }
            }
            PowerMode::HighPerformance => {
                // High performance prefers parallelism as soon as overhead is justified.
                if len >= 8192 {
                    Strategy::ParallelSimd(target_threads)
                } else {
                    Strategy::SingleThreadSimd
                }
            }
            _ => { // Balanced
                if len < 32768 {
                    Strategy::SingleThreadSimd
                } else {
                    Strategy::ParallelSimd(target_threads)
                }
            }
        }
    }
}
