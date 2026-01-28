use crate::gpu::{GpuBackend, vulkan::VulkanBackend, opencl::OpenCLBackend, opengl::OpenGLBackend};
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuPolicy {
    Adaptive,
    ForceGpu,
    ForceCpu,
}

static GPU_POLICY: OnceLock<RwLock<GpuPolicy>> = OnceLock::new();

pub fn set_gpu_policy(policy: GpuPolicy) {
    let locker = GPU_POLICY.get_or_init(|| RwLock::new(GpuPolicy::Adaptive));
    if let Ok(mut lock) = locker.write() {
        *lock = policy;
    }
}

pub fn get_gpu_policy() -> GpuPolicy {
    GPU_POLICY.get()
        .and_then(|l| l.read().ok())
        .map(|lock| match *lock {
            GpuPolicy::Adaptive => GpuPolicy::Adaptive,
            GpuPolicy::ForceGpu => GpuPolicy::ForceGpu,
            GpuPolicy::ForceCpu => GpuPolicy::ForceCpu,
        })
        .unwrap_or(GpuPolicy::Adaptive)
}

pub fn select_best_backend() -> Option<Box<dyn GpuBackend>> {
    // Priority: Vulkan > OpenCL > OpenGL
    let v = VulkanBackend;
    if v.is_available() { return Some(Box::new(v)); }
    
    let oc = OpenCLBackend;
    if oc.is_available() { return Some(Box::new(oc)); }
    
    let og = OpenGLBackend;
    if og.is_available() { return Some(Box::new(og)); }
    
    None
}
