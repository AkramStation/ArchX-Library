use archx::{engine, PowerMode, get_system_info};

/// Flagship v2.0 Sovereign Demo
/// 
/// This example demonstrates the unified SystemInfo architecture and 
/// the smarter v2.0 adaptive execution engine.
fn main() {
    println!("ArchX v2.0 - Sovereign Acceleration Ecosystem");
    println!("==============================================");

    // 1. Inspect the Unified v2 System
    let info = get_system_info();
    println!("Device Status:");
    println!(" - CPU: {:?} ({} cores, {} logical)", info.cpu.arch, info.cpu.cores, info.cpu.logical_processors);
    println!(" - GPU: {}", if let Some(gpu) = &info.gpu { format!("{:?} ({}GB)", gpu.api, gpu.memory_gb) } else { "Not Detected / Not Registered".to_string() });
    println!(" - Bitness: {:?}", info.cpu.bits);
    
    let size = 2_000_000;
    let a = vec![0.5f32; size];
    let b = vec![1.5f32; size];
    let mut out = vec![0.0f32; size];

    println!("\nExecuting v2 Adaptive Dispatch (2M elements)...");

    // 2. Use the Fluent API with v2.0 Heuristics
    // The engine automatically decides between Scalar, SIMD (AVX/Neon), 
    // Parallel, or GPU based on the unified info gathered above.
    engine()
        .with_profile(true)
        .with_power_mode(PowerMode::HighPerformance)
        .with_limits(0.8) // Use up to 80% of system resources
        .add(&a, &b, &mut out);

    // 3. Analysis
    let profiler = archx::profiling::get_profiler();
    profiler.print_summary();

    println!("Verification: Result[0] = {} (Expected 2.0)", out[0]);
    println!("==============================================");
    println!("ArchX v2.0: The Sovereignty of Choice.");
}
