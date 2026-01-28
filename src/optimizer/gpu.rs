/// Trait representing a GPU offloading backend.
/// 
/// WHY: This allows ArchX to remain dependency-free while providing 
/// an extensibility point for GPU acceleration (e.g., CUDA, wgpu).
pub trait GpuBackend: Send + Sync {
    /// Returns the name of the backend.
    fn name(&self) -> &str;
    /// Executes the add operation on the GPU.
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String>;
    
    /// Optional: Asynchronous implementation for non-blocking GPU dispatch.
    /// Default implementation delegates to synchronous 'add'.
    fn add_async(&self, _a: Vec<f32>, _b: Vec<f32>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<f32>, String>> + Send>> {
        let backend_name = self.name().to_string();
        Box::pin(async move {
            Err(format!("Backend '{}' does not support native async", backend_name))
        })
    }
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

use std::sync::{OnceLock, RwLock};

static ACTIVE_BACKEND: OnceLock<RwLock<Option<Box<dyn GpuBackend>>>> = OnceLock::new();

/// Registers a GPU backend for offloading.
/// 
/// WHY: v0.6 ensures thread-safety by using a RwLock, allowing 
/// concurrent applications (like Tauri) to safely register and use backends.
pub fn register_backend(backend: Box<dyn GpuBackend>) {
    let locker = ACTIVE_BACKEND.get_or_init(|| RwLock::new(None));
    if let Ok(mut lock) = locker.write() {
        *lock = Some(backend);
    }
}

/// Returns the name of the currently active GPU backend, if any.
pub fn get_active_backend_name() -> Option<String> {
    ACTIVE_BACKEND.get()
        .and_then(|l| l.read().ok())
        .and_then(|lock| lock.as_ref().map(|b| b.name().to_string()))
}

/// Internal helper to access the backend for computation.
pub(crate) fn with_backend<F, R>(f: F) -> Option<R> 
where 
    F: FnOnce(&dyn GpuBackend) -> R 
{
    ACTIVE_BACKEND.get()
        .and_then(|l| l.read().ok())
        .and_then(|lock| lock.as_ref().map(|b| f(b.as_ref())))
}
