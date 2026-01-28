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
        // 1. Check for manual GPU preference + minimal size threshold
        if hints.prefer_gpu && len >= 100_000 {
            return Strategy::GpuOffload;
        }

        // 2. Power Saving Mode: Favor SIMD over massive threading
        if let PowerMode::PowerSaving = hints.power_mode {
            if len < 500_000 {
                return Strategy::SingleThreadSimd;
            }
        }

        // 3. Size-based heuristics
        if len < 16384 && hints.thread_count.is_none() {
            Strategy::SingleThreadSimd
        } else if len < 1000 {
            Strategy::ScalarFallback
        } else {
            // Calculate dynamic thread count based on max_cpu_usage
            let available = info.logical_processors;
            let target = if let Some(cap) = hints.max_cpu_usage {
                (available as f32 * cap).max(1.0) as usize
            } else {
                available
            };
            
            Strategy::ParallelSimd(target.min(available))
        }
    }
}
