use archx::{engine, PowerMode};

/// Flagship Example: Professional Fluent API
/// 
/// This example demonstrates the v2.0 Sovereign features:
/// 1. Fluent Builder API
/// 2. Automatic Heuristic Routing
/// 3. Human-Readable Performance Profiling
fn main() {
    println!("ArchX v2.0 - Sovereign Acceleration Ecosystem");
    println!("=====================================================");

    let size = 1_000_000;
    let a = vec![1.5f32; size];
    let b = vec![2.5f32; size];
    let mut out = vec![0.0f32; size];

    println!("Executing high-priority compute with resource limits...");

    // Professional API: Fluent, safe, and descriptive.
    engine()
        .with_profile(true)                // Enable internal diagnostics
        .with_limits(0.5)                  // Cap CPU at 50% utilization
        .with_power_mode(PowerMode::HighPerformance)
        .add(&a, &b, &mut out);

    // After execution, we can see exactly what the engine decided.
    archx::profiling::get_profiler().print_summary();

    println!("Verification: Result[0] = {} (Expected 4.0)", out[0]);
    println!("=====================================================");
    println!("ArchX Ecosystem: Smart. Fast. Professional.");
}
