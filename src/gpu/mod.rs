use std::sync::{OnceLock, RwLock};

pub trait GpuBackend: Send + Sync {
    /// Returns true if the backend is available on the current platform.
    fn is_available(&self) -> bool;

    /// Executes calculation (Sovereign v3.0 Interface).
    fn execute(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        let mut out = vec![0.0; a.len()];
        let _ = self.add(a, b, &mut out);
        out
    }

    /// Optimized slice-based addition.
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String>;

    /// Optimized slice-based subtraction.
    fn sub(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String>;

    /// Optimized slice-based multiplication.
    fn mul(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String>;

    /// Optimized dot product.
    fn dot(&self, a: &[f32], b: &[f32]) -> Result<f32, String>;

    /// Optional: Asynchronous implementation for non-blocking GPU dispatch.
    fn add_async(&self, _a: Vec<f32>, _b: Vec<f32>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<f32>, String>> + Send>> {
        let name = self.name().to_string();
        Box::pin(async move {
            Err(format!("Backend '{}' does not support native async", name))
        })
    }
    
    /// Returns the name of the backend (e.g., "Vulkan", "OpenCL").
    fn name(&self) -> &'static str;
}

pub mod vulkan;
pub mod opencl;
pub mod opengl;
pub mod manager;

pub use manager::{GpuPolicy, set_gpu_policy, get_gpu_policy, select_best_backend};

pub struct DisabledBackend;

impl GpuBackend for DisabledBackend {
    fn is_available(&self) -> bool { false }
    fn add(&self, _: &[f32], _: &[f32], _: &mut [f32]) -> Result<(), String> {
        Err("GPU Backend is disabled or unavailable.".to_string())
    }
    fn sub(&self, _: &[f32], _: &[f32], _: &mut [f32]) -> Result<(), String> {
        Err("GPU Backend is disabled or unavailable.".to_string())
    }
    fn mul(&self, _: &[f32], _: &[f32], _: &mut [f32]) -> Result<(), String> {
        Err("GPU Backend is disabled or unavailable.".to_string())
    }
    fn dot(&self, _: &[f32], _: &[f32]) -> Result<f32, String> {
        Err("GPU Backend is disabled or unavailable.".to_string())
    }
    fn name(&self) -> &'static str { "Disabled" }
}

static ACTIVE_BACKEND: OnceLock<RwLock<Option<Box<dyn GpuBackend>>>> = OnceLock::new();

pub fn register_backend(backend: Box<dyn GpuBackend>) {
    let locker = ACTIVE_BACKEND.get_or_init(|| RwLock::new(None));
    if let Ok(mut lock) = locker.write() {
        *lock = Some(backend);
    }
}

pub fn get_active_backend_name() -> Option<String> {
    ACTIVE_BACKEND.get()
        .and_then(|l| l.read().ok())
        .and_then(|lock| lock.as_ref().map(|b| b.name().to_string()))
}

/// Executes a closure with the currently active GPU backend.
pub fn with_backend<F, R>(f: F) -> Option<R> 
where 
    F: FnOnce(&dyn GpuBackend) -> R 
{
    ACTIVE_BACKEND.get()
        .and_then(|l| l.read().ok())
        .and_then(|lock| lock.as_ref().map(|b| f(b.as_ref())))
}

/// Convenience: Executes addition on the active GPU backend.
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String> {
    with_backend(|backend: &dyn GpuBackend| backend.add(a, b, out))
        .unwrap_or_else(|| Err("No GPU backend registered".to_string()))
}

/// Convenience: Executes subtraction on the active GPU backend.
pub fn sub(a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String> {
    with_backend(|backend: &dyn GpuBackend| backend.sub(a, b, out))
        .unwrap_or_else(|| Err("No GPU backend registered".to_string()))
}

/// Convenience: Executes multiplication on the active GPU backend.
pub fn mul(a: &[f32], b: &[f32], out: &mut [f32]) -> Result<(), String> {
    with_backend(|backend: &dyn GpuBackend| backend.mul(a, b, out))
        .unwrap_or_else(|| Err("No GPU backend registered".to_string()))
}

/// Convenience: Executes dot product on the active GPU backend.
pub fn dot(a: &[f32], b: &[f32]) -> Result<f32, String> {
    with_backend(|backend: &dyn GpuBackend| backend.dot(a, b))
        .unwrap_or_else(|| Err("No GPU backend registered".to_string()))
}

/// Convenience: Executes async addition on the active GPU backend.
pub fn add_async(a: Vec<f32>, b: Vec<f32>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<f32>, String>> + Send>> {
    let res = with_backend(|backend: &dyn GpuBackend| backend.add_async(a.clone(), b.clone()));
    
    if let Some(fut) = res {
        fut
    } else {
        Box::pin(async { Err("No GPU backend registered".to_string()) })
    }
}
