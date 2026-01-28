//! ArchX: CPU-aware detection and optimization foundation.
//! 
//! v0.1 focuses on a clean modular architecture and scalar fallback.

pub mod cpu;
pub mod dispatch;
pub mod optimizer;
pub mod system;
pub mod diagnostics;
pub mod integration;
pub mod plugin;
pub mod async_ops;
pub mod profiling;
pub mod hardware;
pub mod adaptive;

/// Public API gateway for common operations.
pub use system::{add, add_advanced, get_info, WorkloadHints};
pub use optimizer::scheduler::PowerMode;
pub use async_ops::add_async;
pub use optimizer::gpu::{register_backend, GpuBackend};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_detection() {
        let info = get_info();
        println!("Detected CPU: {:?}", info);
        // Sane check: Arch shouldn't be Unknown on most dev machines
        assert_ne!(info.arch, cpu::arch::CpuArch::Unknown);
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


