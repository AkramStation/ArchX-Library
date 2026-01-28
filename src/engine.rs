use crate::system::WorkloadHints;
use crate::optimizer::scheduler::PowerMode;
use crate::profiling::get_profiler;

/// The entry point for the professional ArchX ecosystem.
/// 
/// `ArchXEngine` provides a fluent, builder-style API for configuring
/// and executing performance-critical workloads with fine-grained control.
///
/// # Example
///
/// ```rust
/// use archx::{engine, PowerMode};
///
/// let a = vec![1.0; 100];
/// let b = vec![2.0; 100];
/// let mut out = vec![0.0; 100];
///
/// engine()
///     .with_limits(0.5) // Cap at 50% CPU
///     .with_power_mode(PowerMode::HighPerformance)
///     .add(&a, &b, &mut out);
/// ```
pub struct ArchXEngine {
    hints: WorkloadHints,
    profiling_enabled: bool,
}

impl ArchXEngine {
    /// Creates a new engine instance with default heuristics.
    pub fn new() -> Self {
        Self {
            hints: WorkloadHints::default(),
            profiling_enabled: false,
        }
    }

    /// Toggles internal profiling for this execution.
    /// When enabled, performance metrics are recorded and can be exported.
    pub fn with_profile(mut self, enabled: bool) -> Self {
        self.profiling_enabled = enabled;
        self
    }

    /// Sets a hard limit on CPU core utilization (0.0 to 1.0).
    /// 
    /// # Scenarios
    /// - Set to `0.2` for background tasks to avoid UI lag.
    /// - Set to `1.0` for maximum throughput in batch processing.
    pub fn with_limits(mut self, cpu_usage: f32) -> Self {
        self.hints.max_cpu_usage = Some(cpu_usage.clamp(0.0, 1.0));
        self
    }

    /// Manually specifies the power profile for heuristics.
    /// 
    /// Different power modes allow the adaptive engine to choose between
    /// aggressive threading or cache-efficient SIMD execution.
    pub fn with_power_mode(mut self, mode: PowerMode) -> Self {
        self.hints.power_mode = mode;
        self
    }

    /// Forces a specific thread count, bypassing automatic detection.
    /// 
    /// **CAUTION**: Using more threads than the physical hardware supports 
    /// may lead to cache thrashing and degraded performance.
    pub fn with_threads(mut self, count: usize) -> Self {
        self.hints.thread_count = Some(count);
        self
    }

    /// Executes a parallel addition using the configured engine state.
    /// This is the primary execution method for the engine.
    pub fn add(&self, a: &[f32], b: &[f32], out: &mut [f32]) {
        if self.profiling_enabled {
            get_profiler().set_enabled(true);
        }
        
        crate::system::add_advanced(a, b, out, self.hints.clone());
    }
}

/// A convenient constructor for the [`ArchXEngine`].
pub fn engine() -> ArchXEngine {
    ArchXEngine::new()
}
