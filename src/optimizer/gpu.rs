/// Trait representing a GPU offloading backend.
/// 
/// WHY: This allows ArchX to remain dependency-free while providing 
/// an extensibility point for GPU acceleration (e.g., CUDA, wgpu).
pub trait GpuBackend: Send + Sync {
    /// Returns the name of the backend.
    fn name(&self) -> &str;
    /// Executes the add operation on the GPU.
    fn add(&self, a: &[f32], b: & [f32], out: &mut [f32]) -> Result<(), String>;
}

/// Mock GPU backend for ArchX v0.5 demonstration.
/// 
/// WHY: Provides a safe fallback and proof-of-concept for the offload architecture.
pub struct MockGpuBackend;

impl GpuBackend for MockGpuBackend {
    fn name(&self) -> &str { "MockCPUParallel" }
    
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String> {
        // In a real implementation, this would involve buffer mapping and kernel dispatch.
        // For the mock, we delegate to parallel CPU implementation.
        crate::optimizer::parallel::add_parallel_impl(a, b, out, &crate::optimizer::scheduler::WorkloadHints::default());
        Ok(())
    }
}

static mut ACTIVE_BACKEND: Option<Box<dyn GpuBackend>> = None;

/// Registers a GPU backend for offloading.
pub fn register_backend(backend: Box<dyn GpuBackend>) {
    // SAFETY: Simple global assignment for v0.5. 
    // In production, this would use a Mutex or Atomic.
    unsafe {
        ACTIVE_BACKEND = Some(backend);
    }
}

/// Returns the currently active GPU backend, if any.
pub fn get_backend() -> Option<&'static dyn GpuBackend> {
    unsafe {
        ACTIVE_BACKEND.as_ref().map(|b| b.as_ref())
    }
}
