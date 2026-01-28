//! ArchX: CPU-aware detection and optimization foundation.
//! 
//! v0.1 focuses on a clean modular architecture and scalar fallback.

pub mod cpu;
pub mod dispatch;
pub mod optimizer;
pub mod system;

/// Public API gateway for common operations.
pub use system::{add, get_info};

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
    fn test_add_operation() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let mut out = vec![0.0; 4];
        
        add(&a, &b, &mut out);
        
        assert_eq!(out, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_simd_vs_scalar_consistency() {
        use crate::optimizer::scalar;
        use crate::optimizer::simd::{sse2, avx, avx2};

        let size = 1025; // Intentional remainder
        let a = vec![1.5f32; size];
        let b = vec![2.5f32; size];
        let mut out_scalar = vec![0.0f32; size];
        
        scalar::add_impl(&a, &b, &mut out_scalar);
        
        #[cfg(target_arch = "x86_64")]
        {
            let mut out_sse2 = vec![0.0f32; size];
            let mut out_avx = vec![0.0f32; size];
            let mut out_avx2 = vec![0.0f32; size];

            sse2::add_sse2_impl(&a, &b, &mut out_sse2);
            assert_eq!(out_scalar, out_sse2, "SSE2 and Scalar results must match");

            avx::add_avx_impl(&a, &b, &mut out_avx);
            assert_eq!(out_scalar, out_avx, "AVX and Scalar results must match");

            avx2::add_avx2_impl(&a, &b, &mut out_avx2);
            assert_eq!(out_scalar, out_avx2, "AVX2 and Scalar results must match");
        }
    }

    #[test]
    #[ignore]
    fn benchmark_add() {
        use std::time::Instant;
        
        let size = 1_000_000;
        let a = vec![1.0f32; size];
        let b = vec![2.0f32; size];
        let mut out = vec![0.0f32; size];

        // Warm up and initialize dispatch
        add(&a, &b, &mut out);

        let start = Instant::now();
        for _ in 0..100 {
            add(&a, &b, &mut out);
        }
        let duration = start.elapsed();
        
        // Use system::get_info for display
        let info = system::get_info();

        println!("\n--- ArchX v0.3 Benchmark ---");
        println!("Size: {} floats", size);
        println!("Arch: {:?}", info.arch);
        println!("AVX2 support: {}", info.features.avx2);
        println!("AVX  support: {}", info.features.avx);
        println!("SSE2 support: {}", info.features.sse2);
        println!("Total time (100 iterations): {:?}", duration);
        println!("Average per iteration: {:?}", duration / 100);
        println!("----------------------------\n");
    }
}

