use archx::{engine, PowerMode};

/// v2.1 Sovereign WASM Benchmark Simulation
/// 
/// Demonstrates how ArchX v2.1 handles environment-aware dispatch 
/// in resource-constrained environments like WASM or background workers.
fn main() {
    println!("ArchX v2.1 - WASM / Background Benchmarker");
    println!("===========================================");

    let sizes = [10_000, 100_000, 1_000_000];
    
    for &size in &sizes {
        println!("\n--- Size: {} elements ---", size);
        let a = vec![1.0f32; size];
        let b = vec![2.0f32; size];
        let mut out = vec![0.0f32; size];

        // Background worker simulation: limited CPU, power saving
        engine()
            .with_profile(true)
            .with_power_mode(PowerMode::PowerSaving)
            .with_limits(0.2) // Cap at 20% to stay responsive
            .add(&a, &b, &mut out);

        archx::profiling::get_profiler().print_summary();
        archx::profiling::get_profiler().clear();
    }

    println!("===========================================");
    println!("ArchX v2.1: Efficient on any edge.");
}
