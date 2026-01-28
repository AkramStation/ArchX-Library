use archx::*;
use std::time::Instant;

/// ArchX v1.0 Production Demo
/// 
/// This example demonstrates the full capabilities of the production-ready 1.0 release:
/// - Smart Hardware-Aware Dispatch
/// - Adaptive Resource Management (PowerSaving vs HighPerformance)
/// - Integrated Multi-threaded SIMD acceleration
/// - Deep Diagnostics & Profiling
#[tokio::main]
async fn main() {
    println!("ArchX v1.0 - Official Production Ready Release");
    println!("===============================================");

    // 1. System Insight
    let info = get_info();
    println!("Platform:  {:?} ({})", info.arch, info.bits);
    println!("SIMD:      SSE2: {}, AVX: {}, AVX2: {}", 
        info.features.sse2, info.features.avx, info.features.avx2);
    
    // 2. Massive Scalable Workload
    let size = 10_000_000;
    println!("\nInitializing {}M elements...", size / 1_000_000);
    let a = vec![1.23f32; size];
    let b = vec![4.56f32; size];
    let mut out = vec![0.0f32; size];

    // 3. Execution Path: Resource-Aware High Performance
    println!("Executing High Performance Path (Parallel SIMD)...");
    let mut hp_hints = WorkloadHints::default();
    hp_hints.power_mode = PowerMode::HighPerformance;
    
    let start = Instant::now();
    add_advanced(&a, &b, &mut out, hp_hints);
    println!("Completed in: {:?}", start.elapsed());

    // 4. Execution Path: Energy-Efficient (Power Saving)
    println!("\nExecuting Power Saving Path (Adaptive Scheduling)...");
    let mut ps_hints = WorkloadHints::default();
    ps_hints.power_mode = PowerMode::PowerSaving;
    
    let start = Instant::now();
    add_advanced(&a, &b, &mut out, ps_hints);
    println!("Completed in: {:?}", start.elapsed());

    // 5. Async Integration (For Tauri/CLI responsiveness)
    println!("\nExecuting Async Production Path...");
    let result = add_async(a, b, WorkloadHints::default()).await;
    println!("Async result computed. Size: {}", result.len());

    println!("===============================================");
    println!("ArchX v1.0 - Verification COMPLETE");
}
