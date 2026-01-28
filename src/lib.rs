//! # ArchX — Your CPU already knows how to be fast. Let it decide.
//!
//! **ArchX** is a mission-critical, adaptive acceleration engine for the modern Rust ecosystem. 
//! It eliminates the guesswork from performance optimization by dynamically routing workloads 
//! across SIMD, Multithreading, and GPU based on real-time hardware topology.
//!
//! ## 🚀 Quick Start
//!
//! ```rust
//! use archx::add;
//!
//! let a = vec![1.0; 1_000_000];
//! let b = vec![2.0; 1_000_000];
//! let mut out = vec![0.0; 1_000_000];
//!
//! // ArchX detects your CPU and decides the best path (SIMD or Parallel).
//! add(&a, &b, &mut out);
//! ```
//!
//! ## 🏗️ Core Architecture
//!
//! - **Adaptive Engine**: Intelligent heuristics that balance throughput vs. latency.
//! - **Hardware Awareness**: Runtime detection of AVX-512, AVX2, AVX, and SSE2.
//! - **Fluent API**: Builder-style control for resource-capped environments.
//!
//! ## 🛠️ Feature Flags
//!
//! - `serde`: Enables serialization for hardware info and metrics.
//!
//! ---
//! Designed with love by **Codevora Studio**.

pub mod cpu;
pub mod dispatch;
pub mod optimizer;
pub mod system;
pub mod diagnostics;
pub mod integration;
pub mod engine;
pub mod plugin;
pub mod async_ops;
pub mod profiling;
pub mod hardware;
pub mod adaptive;

/// Public API gateway for common operations.
pub use system::{add, add_advanced, get_info, get_system_info, WorkloadHints};
pub use engine::{engine, ArchXEngine};
pub use optimizer::scheduler::PowerMode;
pub use async_ops::add_async;
pub use optimizer::gpu::{register_backend, GpuBackend};
pub use hardware::{SystemInfo, CpuInfo, GpuInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_system_detection() {
        let info = get_system_info();
        println!("Detected System v2: {:?}", info);
        // Sane check: CPU Arch shouldn't be Unknown
        assert_ne!(info.cpu.arch, cpu::arch::CpuArch::Unknown);
    }

    #[test]
    fn test_add_operation_basic() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let mut out = vec![0.0; 4];
        
        add(&a, &b, &mut out);
        
        assert_eq!(out, vec![6.0, 8.0, 10.0, 12.0]);
    }
}


