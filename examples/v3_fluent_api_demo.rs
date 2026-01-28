use archx::{ArchX, Policy, GpuPolicy, MathMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- ArchX v3.0 Sovereign Fluent API Demo ---");
    println!("Unified Compute | Robust Error Handling | High-Performance Ergonomics\n");

    let size = 1_000_000;
    let a = vec![10.0f32; size];
    let b = vec![5.0f32; size];
    let mut out = vec![0.0f32; size];

    // 1. Fluent Math Chain (Hybrid/Adaptive)
    println!("1. Executing chainable hybrid math...");
    ArchX::compute()
        .with_policy(Policy::Performance)
        .with_gpu(GpuPolicy::Adaptive)
        .with_mode(MathMode::Fast)
        .profile(true)
        .add(&a, &b, &mut out)?; // Returns ArchXResult<()>
    
    println!("   Add Result[0]: {} (Expected: 15.0)", out[0]);

    // 2. Functional Reductions
    println!("\n2. Executing chainable dot product...");
    let dot = ArchX::compute()
        .enable_gpu(true)
        .max_threads(4)
        .dot(&a, &b)?; // Returns ArchXResult<f32>
    
    println!("   Dot Product: {} (Expected: {})", dot, (size as f32) * 50.0);

    // 3. Managed Task Execution
    println!("\n3. Executing managed closure task...");
    let sum = ArchX::compute()
        .with_policy(Policy::Balanced)
        .run(|| {
            // Complex multi-step task protected by ArchX heuristics
            let mut total = 0.0;
            for x in &a { total += x; }
            total
        });
    
    println!("   Managed Sum: {} (Expected: {})", sum, (size as f32) * 10.0);

    // 4. Error Handling Example
    println!("\n4. Testing error handling (Misaligned slices)...");
    let _short_out = vec![0.0; 10];
    let _status = ArchX::compute().add(&a, &b, &mut out[..10]); 
    // Wait, the above is actually aligned if I use a slice, let's try mismatched
    let err_status = ArchX::compute().add(&a, &b, &mut out[..10]);
    
    match err_status {
        Ok(_) => println!("   Successfully updated (Wait, this shouldn't happen if lengths mismatched)"),
        Err(e) => println!("   Caught Expected Error: {}", e),
    }

    println!("\nArchX v3.0: Sovereign Compute, Redefined.");
    Ok(())
}
