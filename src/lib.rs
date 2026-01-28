//! # ArchX Sovereign v3.0 — Fluent Performance Runtime
//!
//! `ArchX` is a high-performance, adaptive runtime library designed for systems programming in Rust.
//! It empowers developers to build applications that automatically leverage the best available hardware
//! resources—be it CPU (with SIMD), GPU, or a hybrid combination—while maintaining strict safety and stability.
//!
//! ## Core Features
//! - **Adaptive Dispatch**: Intelligently switches between CPU and GPU based on workload size and hardware capability.
//! - **Sovereign Fluent API**: A unified, chainable interface for configuring and executing computations.
//! - **Safe Math Engine**: Built-in protection against overflows and arithmetic errors with multiple precision modes.
//! - **Built-in Profiling**: Real-time metric collection and reporting (JSON/CSV) for performance tuning.
//! - **Hardware Intelligence**: Advanced detection of SIMD levels (AVX2, AVX-512) and GPU vendor capabilities.
//!
//! ## Quick Start
//!
//! Add `ArchX` to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! archx = "3.0"
//! ```
//!
//! ### Simple Adaptive Task
//! ```rust
//! use archx::ArchX;
//!
//! let result = ArchX::run(|| {
//!     // Complex computation here
//!     (0..1000).sum::<u64>()
//! });
//! println!("Result: {}", result);
//! ```
//!
//! ### Using the Fluent API (v3.0)
//! ```rust
//! use archx::{archx, Policy, GpuPolicy};
//!
//! let a = vec![1.0; 1000];
//! let b = vec![2.0; 1000];
//! let mut out = vec![0.0; 1000];
//!
//! archx()
//!     .with_policy(Policy::Balanced)
//!     .with_gpu(GpuPolicy::Adaptive)
//!     .profile(true)
//!     .add(&a, &b, &mut out).unwrap();
//! ```
//!
//! ## Execution Modes
//!
//! 1. **CPU Only**: Direct execution on host threads, utilizing SIMD where available.
//! 2. **GPU Only**: Force offloading to supported GPU backends.
//! 3. **Hybrid**: Workload-aware splitting between CPU and GPU for maximum throughput.
//! 4. **Async**: Background execution for non-blocking operations.
//!
//! ## Safety and Precision
//! `ArchX` provides three mathematical modes via `MathMode`:
//! - `Safe`: Full overflow checking, returns errors on arithmetic failure.
//! - `Fast`: Unchecked, wrapping arithmetic for maximum performance.
//! - `Balanced`: Saturating arithmetic, providing stable results without errors.

pub mod detect;
pub mod profiler;
pub mod decision;
pub mod runtime;
pub mod public_api;
pub mod math;
pub mod core;

// Maintain existing modules for backward compatibility
pub mod cpu;
pub mod dispatch;
pub mod optimizer;
pub mod system;
pub mod gpu;
pub mod async_ops;
pub mod hardware;
pub mod adaptive;
pub mod error;


// Re-export core v3.0 + legacy API
pub use public_api::ArchX;
pub use public_api::archx::{engine, archx, ArchXBuilder};
pub use public_api::sovereign::SovereignBuilder;
pub use error::{ArchXError, ArchXResult};
pub use decision::Policy;
pub use gpu::GpuPolicy;

// Re-export legacy items for stability
pub use hardware::{SystemInfo, CpuInfo, GpuInfo, GpuApi};
pub use system::{add, add_advanced, get_info, get_system_info, WorkloadHints};
pub use adaptive::AdaptiveEngine;
pub use async_ops::add_async;
pub use math::{SafeMath, ArithmeticResult, MathMode, AdaptiveMath};
pub use profiler::{JsonExporter, CsvExporter, ReportExporter, get_profiler};
pub use runtime::ArchXSched;
pub use optimizer::scheduler::PowerMode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3_detection() {
        let state = detect::HardwareState::capture();
        println!("ArchX Sovereign v3.0 Hardware State: {:?}", state);
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
