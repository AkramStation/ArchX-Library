use crate::hardware::HardwareInfo;
use crate::optimizer::scheduler::{WorkloadHints, PowerMode};

/// The strategy chosen by the adaptive engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    SingleThreadSimd,
    ParallelSimd(usize), // Active thread count
    GpuOffload,
    ScalarFallback,
}

/// A smart engine that decides the best compute path at runtime.
pub struct AdaptiveEngine;

impl AdaptiveEngine {
    /// Decides the best strategy based on data size and system state.
    pub fn choose_strategy(len: usize, hints: &WorkloadHints, info: &HardwareInfo) -> Strategy {
        // v1.0 Production Guard: Memory Check
        if !info.can_handle_dataset(len) {
            println!("[ArchX v1.0 WARNING] Dataset size {} exceeds safe memory limits. Forcing SingleThreadSimd.", len);
            return Strategy::SingleThreadSimd;
        }

        // 1. GPU Path - Refined threshold for PCIe transfer overhead
        let gpu_threshold = if info.features.avx512f { 1_000_000 } else { 250_000 };
        
        if hints.prefer_gpu && len >= gpu_threshold {
            return Strategy::GpuOffload;
        }

        // v1.1: Extremely small datasets should avoid SIMD setup costs.
        if len < 1000 {
            return Strategy::ScalarFallback;
        }

        // 2. Power & Scaling Heuristics
        let base_threads = match hints.power_mode {
            PowerMode::PowerSaving => info.cpu_cores,
            _ => info.logical_processors,
        };

        let mut target_threads = if let Some(cap) = hints.max_cpu_usage {
            (info.logical_processors as f32 * cap).max(1.0) as usize
        } else {
            base_threads
        };
        
        // Ensure we don't exceed the requested thread count if provided manually
        if let Some(manual) = hints.thread_count {
            target_threads = target_threads.min(manual);
        }

        match hints.power_mode {
            PowerMode::PowerSaving => {
                if len < 1_000_000 {
                    Strategy::SingleThreadSimd
                } else {
                    Strategy::ParallelSimd(target_threads.min(info.cpu_cores))
                }
            }
            PowerMode::HighPerformance => {
                if len >= 16384 {
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
