use archx::{ArchX, GpuPolicy, Policy};

fn main() -> archx::ArchXResult<()> {
    println!("--- ArchX Sovereign v3.0 GPU Acceleration Flagship Demo ---");
    println!("Unified Hybrid Compute | Sovereign v3.0 Core Engine\n");

    let core_size = 2_000_000;
    let a = vec![1.0f32; core_size];
    let b = vec![2.0f32; core_size];
    let mut out = vec![0.0f32; core_size];

    // 1. Hybrid Execution (Automatic Split)
    println!("1. Hybrid Execution (Adaptive Split via Core Scheduler)...");
    ArchX::compute()
        .with_policy(Policy::Performance)
        .with_gpu(GpuPolicy::Adaptive)
        .profile(true)
        .add(&a, &b, &mut out)?;
    println!("   Result[0]: {} (Expected: 3.0)", out[0]);

    // 2. Forced GPU Execution
    println!("\n2. Forced GPU Execution (100% GPU offload)...");
    ArchX::compute()
        .with_gpu(GpuPolicy::ForceGpu)
        .profile(true)
        .sub(&a, &b, &mut out)?;
    println!("   Result[0]: {} (Expected: -1.0)", out[0]);

    // 3. Forced CPU Execution (Safety Fallback)
    println!("\n3. Forced CPU Execution (100% SIMD optimized)...");
    ArchX::compute()
        .with_gpu(GpuPolicy::ForceCpu)
        .profile(true)
        .mul(&a, &b, &mut out)?;
    println!("   Result[0]: {} (Expected: 2.0)", out[0]);

    // 4. High-Performance Dot Product (Hybrid)
    println!("\n4. Hybrid Dot Product Reduction...");
    let dot = ArchX::compute()
        .with_gpu(GpuPolicy::Adaptive)
        .profile(true)
        .dot(&a, &b)?;
    println!("   Dot Product: {} (Expected: {})", dot, (core_size as f32) * 2.0);

    println!("\nSovereign v3.0: The Future of High-Performance Rust.");
    Ok(())
}
