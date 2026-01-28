use crate::gpu::{GpuBackend, vulkan::VulkanBackend};
use crate::profiling::ProfileScope;

pub struct HybridScheduler;

impl HybridScheduler {
    /// Dispatches a task cooperatively between CPU and GPU.
    /// In v2.1, this uses a default 70/30 split for large workloads.
    pub fn dispatch(a: &[f32], b: &[f32], out: &mut [f32]) {
        let _scope = ProfileScope::new("Hybrid Dispatch", "Hybrid", "CPU+GPU");
        
        let len = a.len();
        let gpu_split = (len as f32 * 0.7) as usize;
        
        // 1. GPU Part (70%)
        let gpu = VulkanBackend;
        if gpu.is_available() {
            let _gpu_scope = ProfileScope::new("GPU Chunk", "Vulkan", "iGPU");
            let _ = gpu.add(&a[..gpu_split], &b[..gpu_split], &mut out[..gpu_split]);
        } else {
            // Fallback for GPU chunk if GPU fails mid-dispatch
            Self::cpu_fallback(&a[..gpu_split], &b[..gpu_split], &mut out[..gpu_split]);
        }

        // 2. CPU Part (30%)
        let _cpu_scope = ProfileScope::new("CPU Chunk", "SIMD", "CPU");
        Self::cpu_fallback(&a[gpu_split..], &b[gpu_split..], &mut out[gpu_split..]);
    }

    fn cpu_fallback(a: &[f32], b: &[f32], out: &mut [f32]) {
        // Use the standard optimized parallel implementation
        crate::optimizer::parallel::add_parallel_impl(a, b, out, &crate::WorkloadHints::default());
    }
}
