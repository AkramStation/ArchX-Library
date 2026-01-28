//! # ArchX v2.3 — Adaptive Resource Intelligence Runtime
//!
//! Let your hardware decide how to be fast safely.

pub mod detect;
pub mod profiler;
pub mod decision;
pub mod execution;
pub mod runtime;
pub mod public_api;

// Maintain existing modules for backward compatibility
pub mod cpu;
pub mod dispatch;
pub mod optimizer;
pub mod system;
pub mod diagnostics;
pub mod integration;
pub mod plugin;
pub mod device;
pub mod gpu;
pub mod async_ops;
pub mod profiling;
pub mod hardware;
pub mod adaptive;


// Re-export core v2.3 API
pub use public_api::ArchX;
pub use public_api::archx::{engine, archx, ArchXBuilder};
pub use decision::Policy;

// Re-export legacy items for stability
pub use hardware::{SystemInfo, CpuInfo, GpuInfo, GpuApi};
pub use system::{add, add_advanced, get_info, get_system_info, WorkloadHints};
pub use adaptive::AdaptiveEngine;
pub use async_ops::add_async;
pub use optimizer::scheduler::PowerMode;
pub use profiling::get_profiler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_3_detection() {
        let state = detect::HardwareState::capture();
        println!("ArchX v2.3 Hardware State: {:?}", state);
        assert!(state.cpu.logical_threads > 0);
    }

    #[test]
    fn test_archx_run_api() {
        let result = ArchX::run(|| {
            let mut x = 0;
            for i in 0..100 { x += i; }
            x
        });
        assert_eq!(result, 4950);
    }

    #[test]
    fn test_legacy_add_parity() {
        let a = vec![1.0; 100];
        let b = vec![2.0; 100];
        let mut out = vec![0.0; 100];
        add(&a, &b, &mut out);
        assert_eq!(out[0], 3.0);
    }
}
