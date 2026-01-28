use crate::detect::HardwareState;
use crate::decision::{Heuristics, Policy};
use crate::profiler::TaskMetrics;
use crate::system::WorkloadHints;
use crate::optimizer::scheduler::PowerMode;

pub struct ArchX;

impl ArchX {
    /// Simply runs a task with adaptive optimization.
    pub fn run<F, R>(task: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let state = HardwareState::capture();
        let strategy = Heuristics::decide(&state, Policy::Balanced);
        
        let mut _metrics = TaskMetrics::new();
        // For ArchX v2.3, we log the strategy and usage
        println!("[ArchX v2.3] Usage: {:.1}%, Strategy: {:?}", state.cpu.usage, strategy);
        
        let result = task();
        
        _metrics.complete();
        result
    }

    /// Builder style for more control (v2.3).
    pub fn adaptive() -> ArchXBuilder {
        ArchXBuilder::new()
    }

    /// Compatibility with v2.2
    pub fn new() -> ArchXBuilder {
        ArchXBuilder::new()
    }
}

pub struct ArchXBuilder {
    policy: Policy,
    hints: WorkloadHints,
    profiling_enabled: bool,
}

impl ArchXBuilder {
    pub fn new() -> Self {
        Self { 
            policy: Policy::Balanced,
            hints: WorkloadHints::default(),
            profiling_enabled: false,
        }
    }

    pub fn with_policy(mut self, policy: Policy) -> Self {
        self.policy = policy;
        self.hints.policy = policy;
        self
    }

    pub fn policy(self, policy: Policy) -> Self {
        self.with_policy(policy)
    }

    pub fn with_profile(mut self, enabled: bool) -> Self {
        self.profiling_enabled = enabled;
        self
    }

    pub fn profile(self, enabled: bool) -> Self {
        self.with_profile(enabled)
    }

    pub fn with_power_mode(mut self, mode: PowerMode) -> Self {
        self.hints.power_mode = mode;
        self
    }

    pub fn with_limits(mut self, cpu_usage: f32) -> Self {
        self.hints.max_cpu_usage = Some(cpu_usage.clamp(0.0, 1.0));
        self
    }

    pub fn enable_gpu(mut self, enabled: bool) -> Self {
        self.hints.enable_gpu = enabled;
        self.hints.prefer_gpu = enabled;
        self
    }

    pub fn enable_hybrid(mut self, enabled: bool) -> Self {
        self.hints.prefer_hybrid = enabled;
        self
    }

    pub fn task<F, R>(self, task: F) -> TaskBuilder<F> 
    where F: FnOnce() -> R 
    {
        TaskBuilder {
            builder: self,
            task,
        }
    }

    /// Executes the addition operation (Legacy/v2.1 Compatibility).
    pub fn execute(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        if self.profiling_enabled {
            crate::profiling::get_profiler().set_enabled(true);
        }
        crate::system::add_advanced(a, b, out, self.hints);
    }

    /// Legacy alias for execute.
    pub fn add(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        self.execute(a, b, out);
    }

    pub fn run_task<F, R>(self, task: F) -> R 
    where F: FnOnce() -> R 
    {
        let state = HardwareState::capture();
        let strategy = Heuristics::decide(&state, self.policy);
        println!("[ArchX v2.3] Usage: {:.1}%, Adaptive Execution: {:?}", state.cpu.usage, strategy);
        task()
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
        let state = HardwareState::capture();
        let strategy = Heuristics::decide(&state, self.builder.policy);
        println!("[ArchX v2.3] Usage: {:.1}%, Adaptive Execution: {:?}", state.cpu.usage, strategy);
        (self.task)()
    }
}

/// Official factory function for ArchX.
pub fn engine() -> ArchXBuilder {
    ArchXBuilder::new()
}

/// Helper for the fluent API.
pub fn archx() -> ArchXBuilder {
    ArchXBuilder::new()
}
