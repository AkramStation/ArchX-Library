use crate::decision::Policy;

/// The central entry point for the ArchX library.
///
/// `ArchX` provides static methods to quickly run tasks or start building 
/// complex configurations using the Fluent API.
pub struct ArchX;

impl ArchX {
    /// Simply runs a task with automatic adaptive optimization.
    ///
    /// The runtime will analyze the system state and execute the closure 
    /// using the most efficient local strategy (CPU/SIMD).
    ///
    /// # Example
    /// ```rust
    /// use archx::ArchX;
    /// let val = ArchX::run(|| 42);
    /// assert_eq!(val, 42);
    /// ```
    pub fn run<F, R>(task: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        Self::compute().run(task)
    }

    /// Creates a builder for configuring the core ArchX engine (legacy).
    /// Used for setting global policies like memory limits or profiling.
    pub fn adaptive() -> ArchXBuilder {
        ArchXBuilder::new()
    }

    /// Compatibility alias for `adaptive()`.
    pub fn new() -> ArchXBuilder {
        ArchXBuilder::new()
    }

    /// Main entry point for the v3.0 Sovereign Fluent API.
    ///
    /// Provides a unified, chainable interface for high-performance math 
    /// and task execution across CPU, GPU, and Hybrid backends.
    ///
    /// # Example
    /// ```rust
    /// use archx::{ArchX, Policy};
    /// ArchX::compute()
    ///     .with_policy(Policy::Performance)
    ///     .enable_gpu(true)
    ///     .sum(&[1.0, 2.0, 3.0]);
    /// ```
    pub fn compute() -> crate::public_api::sovereign::SovereignBuilder {
        crate::public_api::sovereign::SovereignBuilder::new()
    }

    /// Access the specialized high-performance math engine (legacy).
    pub fn math() -> crate::public_api::math_api::MathBuilder {
        crate::public_api::math_api::MathBuilder::new()
    }
}

/// A configuration builder for the ArchX runtime.
pub struct ArchXBuilder {
    policy: Policy,
    profiling_enabled: bool,
    gpu_enabled: bool,
}

impl ArchXBuilder {
    pub fn new() -> Self {
        Self { 
            policy: Policy::Balanced,
            profiling_enabled: false,
            gpu_enabled: true,
        }
    }

    /// Sets the execution policy for the runtime.
    /// Default is `Policy::Balanced`.
    pub fn with_policy(mut self, policy: Policy) -> Self {
        self.policy = policy;
        self
    }

    /// Fluent alias for `with_policy`.
    pub fn policy(self, policy: Policy) -> Self {
        self.with_policy(policy)
    }

    /// Enables or disables real-time profiling.
    /// If enabled, metrics will be collected for subsequent operations.
    pub fn with_profile(mut self, enabled: bool) -> Self {
        self.profiling_enabled = enabled;
        self
    }

    /// Fluent alias for `with_profile`.
    pub fn profile(self, enabled: bool) -> Self {
        self.with_profile(enabled)
    }

    /// Explicitly enables or disables GPU offloading for this builder's context.
    pub fn enable_gpu(mut self, enabled: bool) -> Self {
        self.gpu_enabled = enabled;
        self
    }

    /// Wraps a closure as an ArchX task with the current builder's configuration.
    pub fn task<F, R>(self, task: F) -> TaskBuilder<F> 
    where F: FnOnce() -> R 
    {
        TaskBuilder {
            builder: self,
            task,
        }
    }

    /// Executes a simple vector addition using legacy dispatch.
    ///
    /// # Safety
    /// All slice lengths must match.
    pub fn execute(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        let _ = self.to_sovereign().add(a, b, out);
    }

    /// Legacy alias for `execute`.
    pub fn add(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        self.execute(a, b, out);
    }

    /// Executes the task closure immediately and returns the result.
    pub fn run_task<F, R>(self, task: F) -> R 
    where F: FnOnce() -> R 
    {
        self.to_sovereign().run(task)
    }

    fn to_sovereign(self) -> crate::public_api::sovereign::SovereignBuilder {
        crate::public_api::ArchX::compute()
            .with_policy(self.policy)
            .profile(self.profiling_enabled)
            .enable_gpu(self.gpu_enabled)
    }
}

pub struct TaskBuilder<F> {
    builder: ArchXBuilder,
    task: F,
}

impl<F, R> TaskBuilder<F> 
where F: FnOnce() -> R
{
    pub fn execute(self) -> R {
        self.builder.to_sovereign().run(self.task)
    }
}

/// Official factory function for ArchX.
pub fn engine() -> ArchXBuilder {
    ArchXBuilder::new()
}

/// Helper for the fluent API.
pub fn archx() -> crate::public_api::sovereign::SovereignBuilder {
    crate::public_api::sovereign::SovereignBuilder::new()
}
