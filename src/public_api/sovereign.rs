use crate::decision::Policy;
use crate::gpu::GpuPolicy;
use crate::math::MathMode;
use crate::error::{ArchXResult, ArchXError};

use crate::core::CoreEngine;
/// The flagship unified builder for ArchX v3.0 (Sovereign Fluent).
///
/// `SovereignBuilder` provides a single, chainable interface for high-performance 
/// compute. It handles hardware state detection, execution policy, and 
/// dispatching work to the optimal backend (CPU/SIMD/GPU/Hybrid).
pub struct SovereignBuilder {
    policy: Policy,
    gpu_policy: GpuPolicy,
    gpu_enabled: bool,
    math_mode: MathMode,
    max_threads: Option<usize>,
    profiling: bool,
    engine: &'static CoreEngine,
}

impl SovereignBuilder {
    pub fn new() -> Self {
        Self {
            policy: Policy::Balanced,
            gpu_policy: GpuPolicy::Adaptive,
            gpu_enabled: true,
            math_mode: MathMode::Balanced,
            max_threads: None,
            profiling: false,
            engine: CoreEngine::global(),
        }
    }

    /// Sets the execution policy (Speed, Balance, Privacy, Throttled).
    ///
    /// The policy influences how the scheduler balances performance vs thermal/power efficiency.
    pub fn with_policy(mut self, policy: Policy) -> Self {
        self.policy = policy;
        self.engine.scheduler.set_policy(policy);
        self
    }

    /// Configures GPU compute policy.
    ///
    /// Available modes: `Adaptive`, `ForceGpu`, `ForceCpu`.
    pub fn with_gpu(mut self, policy: GpuPolicy) -> Self {
        self.gpu_policy = policy;
        self.gpu_enabled = policy != GpuPolicy::ForceCpu;
        self.engine.scheduler.set_gpu_policy(policy);
        self
    }

    /// Explicitly enables or disables GPU offloading.
    pub fn enable_gpu(mut self, enabled: bool) -> Self {
        self.gpu_enabled = enabled;
        self
    }

    /// Sets the arithmetic safety mode.
    ///
    /// - `Safe`: Error on overflow.
    /// - `Fast`: Wrapping arithmetic.
    /// - `Balanced`: Saturating arithmetic.
    pub fn with_mode(mut self, mode: MathMode) -> Self {
        self.math_mode = mode;
        self
    }

    /// Limits the maximum number of CPU threads used for parallel tasks.
    pub fn max_threads(mut self, count: usize) -> Self {
        self.max_threads = Some(count);
        self
    }

    /// Enables detailed profiling and diagnostic reporting for operations.
    ///
    /// When enabled, execution strategies and timing metrics are logged 
    /// or collected by the global profiler.
    pub fn profile(mut self, enabled: bool) -> Self {
        self.profiling = enabled;
        self
    }

    // --- Math Terminal Operations ---

    /// Performs vectorized element-wise addition: `out = a + b`.
    ///
    /// # Errors
    /// Returns `ArchXError::InvalidInput` if slice lengths do not match.
    ///
    /// ```rust
    /// # use archx::archx;
    /// archx().add(&[1.0], &[2.0], &mut [0.0]).unwrap();
    /// ```
    pub fn add(self, a: &[f32], b: &[f32], out: &mut [f32]) -> ArchXResult<()> {
        if a.len() != b.len() || a.len() != out.len() {
            return Err(ArchXError::InvalidInput("Slices must have identical lengths".to_string()));
        }
        self.pre_exec();
        self.engine.engine.add(a, b, out, self.math_mode.into())
            .map_err(|e| ArchXError::ExecutionError(e.to_string()))
    }

    /// Performs vectorized element-wise subtraction: `out = a - b`.
    ///
    /// # Errors
    /// Returns `ArchXError::InvalidInput` if slice lengths do not match.
    pub fn sub(self, a: &[f32], b: &[f32], out: &mut [f32]) -> ArchXResult<()> {
        if a.len() != b.len() || a.len() != out.len() {
            return Err(ArchXError::InvalidInput("Slices must have identical lengths".to_string()));
        }
        self.pre_exec();
        self.engine.engine.sub(a, b, out, self.math_mode.into())
            .map_err(|e| ArchXError::ExecutionError(e.to_string()))
    }

    /// Performs vectorized element-wise multiplication: `out = a * b`.
    ///
    /// # Errors
    /// Returns `ArchXError::InvalidInput` if slice lengths do not match.
    pub fn mul(self, a: &[f32], b: &[f32], out: &mut [f32]) -> ArchXResult<()> {
        if a.len() != b.len() || a.len() != out.len() {
            return Err(ArchXError::InvalidInput("Slices must have identical lengths".to_string()));
        }
        self.pre_exec();
        self.engine.engine.mul(a, b, out, self.math_mode.into())
            .map_err(|e| ArchXError::ExecutionError(e.to_string()))
    }

    /// Performs the dot product of two vectors: `sum(a[i] * b[i])`.
    ///
    /// # Errors
    /// Returns `ArchXError::InvalidInput` if slice lengths do not match.
    pub fn dot(self, a: &[f32], b: &[f32]) -> ArchXResult<f32> {
        if a.len() != b.len() {
            return Err(ArchXError::InvalidInput("Slices must have identical lengths".to_string()));
        }
        self.pre_exec();
        self.engine.engine.dot(a, b, self.math_mode.into())
            .map_err(|e| ArchXError::ExecutionError(e.to_string()))
    }

    /// Performs a parallel sum reduction of a vector.
    ///
    /// Utilizes multi-threaded reduction on CPU.
    pub fn sum(self, a: &[f32]) -> ArchXResult<f32> {
        self.pre_exec();
        self.engine.engine.sum(a, self.math_mode.into())
            .map_err(|e| ArchXError::ExecutionError(e.to_string()))
    }

    // --- Task Terminal Operations ---

    /// Executes a closure within the managed ArchX context.
    ///
    /// This allows non-vectorized tasks to benefit from ArchX policy management
    /// and profiling metrics.
    pub fn run<F, R>(self, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        self.pre_exec();
        self.engine.scheduler.run(f).expect("Task execution failed")
    }

    fn pre_exec(&self) {
        let state = self.engine.hardware.capture_state();
        if self.profiling {
            println!("[ArchX Core v3.0] Policy: {:?}, GPU: {:?}, Mode: {:?}, Memory: {} GB", 
                self.policy, self.gpu_policy, self.math_mode, state.available_memory_gb);
        }
    }
}
