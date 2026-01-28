/// Lightweight diagnostics and profiling for ArchX.
/// 
/// WHY: v0.5 introduces internal telemetry to help developers understand 
/// where time is spent (e.g., dispatch vs execution vs sync).
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    pub dispatch_ns: u64,
    pub execution_ns: u64,
    pub thread_sync_ns: u64,
}

#[macro_export]
macro_rules! profile_section {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let res = $block;
        let elapsed = start.elapsed();
        // In a real implementation, this would log to a thread-local or global collector.
        // For v0.5, we'll keep it as a placeholder for the architecture.
        res
    }};
}
