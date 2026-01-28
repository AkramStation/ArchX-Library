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
        if hints.prefer_gpu && len >= 250_000 {
            return Strategy::GpuOffload;
        }

        // 2. Power & Scaling Heuristics
        match hints.power_mode {
            PowerMode::PowerSaving => {
                if len < 1_000_000 {
                    Strategy::SingleThreadSimd
                } else {
                    Strategy::ParallelSimd(info.cpu_cores) // Use physical cores only
                }
            }
            PowerMode::HighPerformance => {
                if len >= 16384 {
                    Strategy::ParallelSimd(info.logical_processors)
                } else {
                    Strategy::SingleThreadSimd
                }
            }
            _ => { // Balanced
                if len < 32768 {
                    Strategy::SingleThreadSimd
                } else {
                    Strategy::ParallelSimd(info.logical_processors)
                }
            }
        }
    }
}
