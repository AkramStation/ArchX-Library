use crate::gpu::{self, GpuBackend, GpuPolicy};

pub struct HybridScheduler;

impl HybridScheduler {
    /// Dispatches a task cooperatively between CPU and GPU.
    pub fn dispatch_add(a: &[f32], b: &[f32], out: &mut [f32]) {
        let (gpu_part, cpu_part) = Self::split(a.len());
        
        if let Some((gpu, len)) = gpu_part {
            let _ = gpu.add(&a[..len], &b[..len], &mut out[..len]);
        }
        
        if let Some((start, end)) = cpu_part {
            Self::cpu_add(&a[start..end], &b[start..end], &mut out[start..end]);
        }
    }

    pub fn dispatch_sub(a: &[f32], b: &[f32], out: &mut [f32]) {
        let (gpu_part, cpu_part) = Self::split(a.len());
        
        if let Some((gpu, len)) = gpu_part {
            let _ = gpu.sub(&a[..len], &b[..len], &mut out[..len]);
        }
        
        if let Some((start, end)) = cpu_part {
            Self::cpu_sub(&a[start..end], &b[start..end], &mut out[start..end]);
        }
    }

    pub fn dispatch_mul(a: &[f32], b: &[f32], out: &mut [f32]) {
        let (gpu_part, cpu_part) = Self::split(a.len());
        
        if let Some((gpu, len)) = gpu_part {
            let _ = gpu.mul(&a[..len], &b[..len], &mut out[..len]);
        }
        
        if let Some((start, end)) = cpu_part {
            Self::cpu_mul(&a[start..end], &b[start..end], &mut out[start..end]);
        }
    }

    pub fn dispatch_dot(a: &[f32], b: &[f32]) -> f32 {
        let (gpu_part, cpu_part) = Self::split(a.len());
        let mut total = 0.0;
        
        if let Some((gpu, len)) = gpu_part {
            total += gpu.dot(&a[..len], &b[..len]).unwrap_or(0.0);
        }
        
        if let Some((start, end)) = cpu_part {
            total += Self::cpu_dot(&a[start..end], &b[start..end]);
        }
        total
    }

    fn split(len: usize) -> (Option<(Box<dyn GpuBackend>, usize)>, Option<(usize, usize)>) {
        let policy = gpu::get_gpu_policy();
        
        match policy {
            GpuPolicy::ForceCpu => (None, Some((0, len))),
            GpuPolicy::ForceGpu => {
                if let Some(gpu) = gpu::select_best_backend() {
                    (Some((gpu, len)), None)
                } else {
                    (None, Some((0, len)))
                }
            }
            GpuPolicy::Adaptive => {
                // Intelligent split: GPU takes 70% if len > 1M, else CPU
                if len > 1_000_000 {
                    if let Some(gpu) = gpu::select_best_backend() {
                        let gpu_len = (len as f32 * 0.7) as usize;
                        (Some((gpu, gpu_len)), Some((gpu_len, len)))
                    } else {
                        (None, Some((0, len)))
                    }
                } else {
                    (None, Some((0, len)))
                }
            }
        }
    }

    fn cpu_add(a: &[f32], b: &[f32], out: &mut [f32]) {
        crate::optimizer::parallel::add_parallel_impl(a, b, out, &crate::WorkloadHints::default());
    }

    fn cpu_sub(a: &[f32], b: &[f32], out: &mut [f32]) {
        crate::runtime::ArchXSched::parallel_sub(a, b, out);
    }

    fn cpu_mul(a: &[f32], b: &[f32], out: &mut [f32]) {
        crate::runtime::ArchXSched::parallel_mul(a, b, out);
    }

    fn cpu_dot(a: &[f32], b: &[f32]) -> f32 {
        crate::runtime::ArchXSched::parallel_dot(a, b)
    }
}
